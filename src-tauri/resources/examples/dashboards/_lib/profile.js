/* Auto-profiler: infer each field's type + cardinality from a record array,
   then suggest a sensible default chart mapping ("Show Me" style).
   Exposes window.MP.{profileRows, suggestMapping, coerce, isDateish}. */
(function () {
  var MP = (window.MP = window.MP || {});

  var DATE_RE = /^\s*(\d{4}[-/]\d{1,2}[-/]\d{1,2}([ T]\d{1,2}:\d{2}(:\d{2})?)?|\d{1,2}[-/]\d{1,2}[-/]\d{2,4})\s*$/;

  function isDateish(v) {
    if (v instanceof Date) return !isNaN(v);
    if (typeof v !== "string") return false;
    if (!DATE_RE.test(v)) return false;
    return !isNaN(Date.parse(v));
  }
  function isNumish(v) {
    if (typeof v === "number") return isFinite(v);
    if (typeof v !== "string") return false;
    var s = v.replace(/[$,%\s]/g, "");
    return s !== "" && isFinite(Number(s));
  }
  function toNum(v) {
    if (typeof v === "number") return v;
    return Number(String(v).replace(/[$,%\s]/g, ""));
  }
  // Coerce a raw cell to the column's inferred type for plotting/aggregation.
  function coerce(v, type) {
    if (v == null || v === "") return null;
    if (type === "number") return isNumish(v) ? toNum(v) : null;
    if (type === "date")   return isDateish(v) ? +new Date(v) : null;
    return String(v);
  }

  function profileRows(rows) {
    if (!rows || !rows.length) return [];
    // Union of keys, preserving first-seen order.
    var keys = [], seen = {};
    rows.forEach(function (r) {
      Object.keys(r).forEach(function (k) { if (!seen[k]) { seen[k] = 1; keys.push(k); } });
    });
    return keys.map(function (k) {
      var nonNull = 0, nDate = 0, nNum = 0, distinct = {}, sample = null, min = Infinity, max = -Infinity;
      for (var i = 0; i < rows.length; i++) {
        var v = rows[i][k];
        if (v == null || v === "") continue;
        nonNull++;
        if (sample == null) sample = v;
        distinct[String(v)] = 1;
        if (isDateish(v)) { nDate++; }
        if (isNumish(v)) { var n = toNum(v); nNum++; if (n < min) min = n; if (n > max) max = n; }
      }
      var type = "category";
      if (nonNull > 0 && nDate / nonNull >= 0.8) type = "date";
      else if (nonNull > 0 && nNum / nonNull >= 0.8) type = "number";
      return {
        name: k, type: type,
        cardinality: Object.keys(distinct).length,
        nonNull: nonNull, sample: sample,
        min: type === "number" ? min : null,
        max: type === "number" ? max : null
      };
    });
  }

  // Pick default X / measures / group / chart type from a profile.
  function suggestMapping(profile) {
    var dates = profile.filter(function (c) { return c.type === "date"; });
    var nums  = profile.filter(function (c) { return c.type === "number"; });
    var cats  = profile.filter(function (c) { return c.type === "category" && c.cardinality > 1; });

    var x, chart;
    if (dates.length) { x = dates[0]; chart = "line"; }
    else if (cats.length) {
      // lowest-cardinality reasonable category
      var byCard = cats.slice().sort(function (a, b) { return a.cardinality - b.cardinality; });
      x = byCard.find(function (c) { return c.cardinality <= 50; }) || byCard[0];
      chart = "bar";
    } else if (nums.length >= 2) { x = nums[0]; chart = "scatter"; }
    else { x = profile[0]; chart = "bar"; }

    var measures = nums.filter(function (c) { return c.name !== x.name; }).map(function (c) { return c.name; });
    if (chart === "scatter") measures = [nums[1].name];
    // group: a low-cardinality category distinct from x (2..8 series)
    var group = "";
    var g = cats.filter(function (c) { return c.name !== x.name && c.cardinality >= 2 && c.cardinality <= 8; })
                .sort(function (a, b) { return a.cardinality - b.cardinality; })[0];
    // only auto-group if we have exactly one measure (else it's too busy)
    if (g && measures.length <= 1) group = g.name;

    return { x: x ? x.name : "", y: measures.slice(0, group ? 1 : 4), group: group, chart: chart };
  }

  MP.isDateish = isDateish;
  MP.coerce = coerce;
  MP.profileRows = profileRows;
  MP.suggestMapping = suggestMapping;
})();
