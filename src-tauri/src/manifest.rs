//! The on-disk layout of apps.json, and the one place that understands both layouts.
//!
//! Current layout: apps nested under their group, groups in display order.
//!
//! ```json
//! { "groups": [ { "name": "Docs", "apps": [ { "id": "...", "name": "..." } ] } ] }
//! ```
//!
//! Older layout: a flat array where every app carries its own `"group"` field. It is still
//! accepted everywhere a manifest is read (the file, `write-config`, the rollback ring), so
//! nothing that holds an old file or an older agent's edit breaks. [`parse`] converts it;
//! [`to_text`] always writes the current layout; [`is_legacy`] tells a caller that the text it
//! read should be rewritten.
//!
//! In memory the app keeps a flat list where each entry has a `group` (the sidebar and every
//! command work from that), plus the ordered list of group names. The names list is what
//! holds group order and the groups that have no apps.

use std::collections::HashSet;

use serde_json::Value;

use crate::AppEntry;

/// What a manifest holds: the apps, and every group name in display order (empty groups too).
pub(crate) struct Layout {
    pub apps: Vec<AppEntry>,
    pub groups: Vec<String>,
}

/// Whether `text` is the older flat-array layout.
pub(crate) fn is_legacy(text: &str) -> bool {
    text.trim_start_matches('\u{feff}').trim_start().starts_with('[')
}

/// Group names in display order: `listed` first, then any group an app names that is not
/// listed, in order of first appearance. Blank and repeated names are dropped.
pub(crate) fn group_order(listed: &[String], apps: &[AppEntry]) -> Vec<String> {
    let mut seen = HashSet::new();
    listed
        .iter()
        .map(|g| g.trim())
        .chain(apps.iter().map(|a| a.group.trim()))
        .filter(|g| !g.is_empty() && seen.insert(g.to_string()))
        .map(str::to_string)
        .collect()
}

/// Read either layout. This is the only migration code: the flat layout is accepted as-is
/// (each entry already names its group, and groups are ordered by first appearance).
pub(crate) fn parse(text: &str) -> Result<Layout, String> {
    let text = text.trim_start_matches('\u{feff}');
    if is_legacy(text) {
        let apps: Vec<AppEntry> = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let groups = group_order(&[], &apps);
        return Ok(Layout { apps, groups });
    }
    let root: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let groups = root
        .get("groups")
        .and_then(Value::as_array)
        .ok_or("expected {\"groups\": [{\"name\": \"...\", \"apps\": [...]}]}")?;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    let mut names = Vec::new();
    for (g, group) in groups.iter().enumerate() {
        let name = group
            .get("name")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .ok_or_else(|| format!("group {} needs a non-empty \"name\"", g + 1))?;
        if !seen.insert(name.to_string()) {
            return Err(format!("duplicate group {name:?}"));
        }
        names.push(name.to_string());
        let apps = group
            .get("apps")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("group {name:?} needs an \"apps\" list"))?;
        for (a, app) in apps.iter().enumerate() {
            let mut object = app
                .as_object()
                .cloned()
                .ok_or_else(|| format!("group {name:?} app {} is not an object", a + 1))?;
            if object.contains_key("group") {
                return Err(format!(
                    "group {name:?} app {}: remove the \"group\" field, an app belongs to the group it is listed under",
                    a + 1
                ));
            }
            object.insert("group".into(), Value::String(name.to_string()));
            out.push(
                serde_json::from_value(Value::Object(object))
                    .map_err(|e| format!("group {name:?} app {}: {e}", a + 1))?,
            );
        }
    }
    Ok(Layout { apps: out, groups: names })
}

/// The current layout: groups in the order [`group_order`] gives (empty ones included), each
/// app's fields in their usual order, without the per-app `group` field.
pub(crate) fn to_text(entries: &[AppEntry], listed: &[String]) -> Result<String, String> {
    let names = group_order(listed, entries);
    let mut apps: Vec<Vec<Value>> = vec![Vec::new(); names.len()];
    for entry in entries {
        let mut value = serde_json::to_value(entry).map_err(|e| e.to_string())?;
        if let Some(object) = value.as_object_mut() {
            object.shift_remove("group");
        }
        // group_order lists every group an app names, so the lookup cannot miss.
        if let Some(slot) = names.iter().position(|n| n.as_str() == entry.group.trim()) {
            apps[slot].push(value);
        }
    }
    let groups: Vec<Value> = names
        .into_iter()
        .zip(apps)
        .map(|(name, apps)| serde_json::json!({ "name": name, "apps": apps }))
        .collect();
    serde_json::to_string_pretty(&serde_json::json!({ "groups": groups })).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLAT: &str = r#"[
      {"id":"a","name":"A","group":"Web apps","type":"web","command":"x","port":1},
      {"id":"b","name":"B","group":"Docs","type":"static","url":"http://x"},
      {"id":"c","name":"C","group":"Web apps","type":"cli","command":"y","note":"n"}
    ]"#;

    fn err(text: &str) -> String {
        match parse(text) {
            Ok(_) => panic!("expected an error for {text}"),
            Err(e) => e,
        }
    }

    #[test]
    fn flat_files_are_detected_and_read() {
        assert!(is_legacy(FLAT));
        assert!(is_legacy("\u{feff}  [ ]"));
        let layout = parse(FLAT).unwrap();
        assert_eq!(layout.apps.len(), 3);
        assert_eq!(layout.apps[2].group, "Web apps");
        assert_eq!(layout.groups, ["Web apps", "Docs"]);
    }

    fn rewrite(text: &str) -> String {
        let layout = parse(text).unwrap();
        to_text(&layout.apps, &layout.groups).unwrap()
    }

    #[test]
    fn migrating_groups_apps_and_keeps_group_and_app_order() {
        let text = rewrite(FLAT);
        assert!(!is_legacy(&text));
        let root: Value = serde_json::from_str(&text).unwrap();
        let groups = root["groups"].as_array().unwrap();
        assert_eq!(groups[0]["name"], "Web apps");
        assert_eq!(groups[1]["name"], "Docs");
        let ids: Vec<&str> = groups[0]["apps"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a["id"].as_str().unwrap())
            .collect();
        assert_eq!(ids, ["a", "c"]);
        assert!(groups[0]["apps"][0].get("group").is_none());
    }

    #[test]
    fn nested_text_round_trips_to_the_same_text() {
        let once = rewrite(FLAT);
        let twice = rewrite(&once);
        assert_eq!(once, twice);
    }

    #[test]
    fn app_fields_keep_their_declared_order() {
        let text = rewrite(FLAT);
        let id = text.find("\"id\"").unwrap();
        let name = text.find("\"name\": \"A\"").unwrap();
        let kind = text.find("\"type\"").unwrap();
        assert!(id < name && name < kind);
    }

    #[test]
    fn nested_apps_get_their_groups_back() {
        let entries = parse(&rewrite(FLAT)).unwrap().apps;
        let by_id: Vec<(&str, &str)> = entries.iter().map(|e| (e.id.as_str(), e.group.as_str())).collect();
        assert_eq!(by_id, [("a", "Web apps"), ("c", "Web apps"), ("b", "Docs")]);
    }

    #[test]
    fn bad_nested_files_say_what_is_wrong() {
        let dup = r#"{"groups":[{"name":"X","apps":[]},{"name":"X","apps":[]}]}"#;
        assert!(err(dup).contains("duplicate group"));
        let own = r#"{"groups":[{"name":"X","apps":[{"id":"a","name":"A","group":"Y","type":"cli","command":"c"}]}]}"#;
        assert!(err(own).contains("remove the \"group\" field"));
        assert!(err(r#"{"groups":[{"apps":[]}]}"#).contains("\"name\""));
        assert!(err(r#"{"groups":[{"name":"X"}]}"#).contains("\"apps\""));
        assert!(parse(r#"{"nope":1}"#).is_err());
        assert!(parse("42").is_err());
    }

    #[test]
    fn empty_groups_keep_their_place_through_a_round_trip() {
        let text = r#"{"groups":[
          {"name":"One","apps":[{"id":"a","name":"A","type":"cli","command":"c"}]},
          {"name":"Empty","apps":[]},
          {"name":"Last","apps":[]}]}"#;
        let layout = parse(text).unwrap();
        assert_eq!(layout.apps.len(), 1);
        assert_eq!(layout.groups, ["One", "Empty", "Last"]);
        let again = parse(&rewrite(text)).unwrap();
        assert_eq!(again.groups, ["One", "Empty", "Last"]);
    }

    #[test]
    fn listed_order_wins_and_apps_in_unlisted_groups_are_appended() {
        let apps = parse(FLAT).unwrap().apps;
        let listed = ["Docs".to_string(), "Empty".to_string(), " Docs ".to_string(), String::new()];
        assert_eq!(group_order(&listed, &apps), ["Docs", "Empty", "Web apps"]);
        let text = to_text(&apps, &listed).unwrap();
        assert_eq!(parse(&text).unwrap().groups, ["Docs", "Empty", "Web apps"]);
    }
}
