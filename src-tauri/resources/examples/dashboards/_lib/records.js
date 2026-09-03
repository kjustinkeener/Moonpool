/* Shared normalizer: turn any parsed JSON / YAML / TOML value into a flat
   record array (array of objects with scalar cell values), so every format
   demo can "handle any shape". Nested objects are flattened with dot-joined
   keys; arrays of scalars become a single JSON cell; deep arrays are skipped.
   Exposes window.MP.toRecords(value).
   ES5-ish (var, no build step), matching the style of profile.js. */
(function () {
  var MP = (window.MP = window.MP || {});

  function isPlainObject(v) {
    return v != null && typeof v === "object" && !Array.isArray(v) && !(v instanceof Date);
  }
  function isScalar(v) {
    return v == null || typeof v === "string" || typeof v === "number" ||
           typeof v === "boolean" || v instanceof Date;
  }
  function isArrayOfObjects(v) {
    if (!Array.isArray(v) || !v.length) return false;
    for (var i = 0; i < v.length; i++) { if (!isPlainObject(v[i])) return false; }
    return true;
  }
  function allValuesObjects(obj) {
    var ks = Object.keys(obj);
    if (!ks.length) return false;
    for (var i = 0; i < ks.length; i++) { if (!isPlainObject(obj[ks[i]])) return false; }
    return true;
  }

  function cell(v) {
    if (v == null) return v;
    if (v instanceof Date) return v.toISOString();
    if (typeof v === "boolean") return String(v);
    // Arrays of scalars collapse into one readable cell; deeper values stringify too.
    if (Array.isArray(v)) {
      try { return JSON.stringify(v); } catch (e) { return String(v); }
    }
    if (typeof v === "object") {
      try { return JSON.stringify(v); } catch (e) { return String(v); }
    }
    return v;
  }

  // Flatten one nested object into a single-level record with dot-joined keys.
  // Nested objects recurse; arrays of scalars become one JSON cell; arrays of
  // objects (a deep array) are skipped so we do not explode a record.
  function flatten(obj, prefix, out) {
    out = out || {};
    Object.keys(obj).forEach(function (k) {
      var key = prefix ? prefix + "." + k : k;
      var v = obj[k];
      if (isPlainObject(v)) {
        flatten(v, key, out);
      } else if (Array.isArray(v)) {
        if (isArrayOfObjects(v)) return; // deep array: skip (keeps the record flat)
        out[key] = cell(v);
      } else {
        out[key] = cell(v);
      }
    });
    return out;
  }

  function flattenRecord(v) {
    if (isPlainObject(v)) return flatten(v, "", {});
    if (isScalar(v)) return { value: cell(v) };
    // A bare array inside a record slot: stringify it.
    return { value: cell(v) };
  }

  MP.toRecords = function (value) {
    if (value == null) return [];

    // 1) Array input.
    if (Array.isArray(value)) {
      if (!value.length) return [];
      if (isArrayOfObjects(value)) {
        return value.map(function (o) { return flattenRecord(o); });
      }
      // Array of scalars (or mixed): wrap each element.
      return value.map(function (x) {
        return isPlainObject(x) ? flattenRecord(x) : { value: cell(x) };
      });
    }

    // 2) Object input.
    if (isPlainObject(value)) {
      // a) First property that holds an array of objects wins (the "table").
      var keys = Object.keys(value);
      for (var i = 0; i < keys.length; i++) {
        if (isArrayOfObjects(value[keys[i]])) {
          return value[keys[i]].map(function (o) { return flattenRecord(o); });
        }
      }
      // b) A map (object of objects): one record per entry, keyed by property.
      if (allValuesObjects(value)) {
        return keys.map(function (k) {
          var rec = flatten(value[k], "", {});
          var withKey = { key: k };
          Object.keys(rec).forEach(function (rk) { withKey[rk] = rec[rk]; });
          return withKey;
        });
      }
      // c) Fallback: a single object becomes a single flattened record.
      return [flatten(value, "", {})];
    }

    // 3) Scalar input.
    return [{ value: cell(value) }];
  };
})();
