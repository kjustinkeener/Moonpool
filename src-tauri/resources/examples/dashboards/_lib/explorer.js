/* MP.createExplorer(rootEl, config) – the reusable "drop your data" explorer.
   config = {
     id:        stable key for localStorage (e.g. "csv"),
     title, subtitle, formatName,          // header + file-button label
     accept:    input accept attr (".csv,.tsv"),
     tabs:      [{ name, rows, mapping? }], // baked, pre-parsed record arrays
     parseFile: (File) => Promise<rows>     // parse a user file to record array
   }
   Depends on echarts + MP.profile (profile.js). Theme "moonpool" must be registered. */
(function () {
  var MP = (window.MP = window.MP || {});
  var CHARTS = [
    { v: "line", t: "Line" }, { v: "area", t: "Area" }, { v: "bar", t: "Bar" },
    { v: "scatter", t: "Scatter" }, { v: "pie", t: "Pie" }
  ];

  function el(tag, cls, html) { var e = document.createElement(tag); if (cls) e.className = cls; if (html != null) e.innerHTML = html; return e; }
  function fmt(n) {
    if (n == null || !isFinite(n)) return "–";
    var a = Math.abs(n);
    if (a >= 1e9) return (n / 1e9).toFixed(1).replace(/\.0$/, "") + "B";
    if (a >= 1e6) return (n / 1e6).toFixed(1).replace(/\.0$/, "") + "M";
    if (a >= 1e3) return (n / 1e3).toFixed(1).replace(/\.0$/, "") + "k";
    return (Math.round(n * 100) / 100).toLocaleString();
  }
  function dateLabel(ts) {
    var d = new Date(ts);
    return d.getUTCHours() || d.getUTCMinutes()
      ? d.toISOString().slice(0, 16).replace("T", " ")
      : d.toISOString().slice(0, 10);
  }

  MP.createExplorer = function (root, config) {
    var tabs = config.tabs.slice();      // may grow with "Your data"
    var active = 0;
    var chartEl, chartTitle, schemaEl, mapStrip, kpiRow, chart, tabsEl;

    // ---- shell ----
    var wrap = el("div", "wrap");
    var head = el("div", "app-head");
    var htext = el("div");
    htext.appendChild(el("h1", "app-title", config.title));
    if (config.subtitle) htext.appendChild(el("p", "app-sub", config.subtitle));
    head.appendChild(htext);
    head.appendChild(el("span", "badge", "◆ " + (config.formatName || "DATA")));
    wrap.appendChild(head);

    kpiRow = el("div", "kpis"); wrap.appendChild(kpiRow);

    tabsEl = el("div", "tabs"); wrap.appendChild(tabsEl);

    mapStrip = el("div", "mapping card"); wrap.appendChild(mapStrip);

    var chartCard = el("div", "card chart-card dropzone");
    chartTitle = el("div", "chart-title");
    schemaEl = el("div", "schema");
    chartEl = el("div", "chart");
    chartCard.appendChild(chartTitle); chartCard.appendChild(schemaEl); chartCard.appendChild(chartEl);
    wrap.appendChild(chartCard);

    // data preview (under the graph): parsed table (left) + raw source text (right)
    var previewGrid = el("div", "preview-grid");
    var tableCard = el("div", "card table-card");
    var tableHead = el("div", "chart-title");
    var tableScroll = el("div", "table-scroll");
    tableCard.appendChild(tableHead); tableCard.appendChild(tableScroll);
    var codeCard = el("div", "card code-card");
    var codeHead = el("div", "chart-title");
    var codeScroll = el("div", "code-scroll");
    codeCard.appendChild(codeHead); codeCard.appendChild(codeScroll);
    previewGrid.appendChild(tableCard); previewGrid.appendChild(codeCard);
    wrap.appendChild(previewGrid);

    var foot = el("div", "foot");
    foot.appendChild(el("span", null, "Moonpool example dashboard · runs offline, no server, no CDN"));
    foot.appendChild(el("span", null, "Charts by Apache ECharts"));
    wrap.appendChild(foot);

    root.appendChild(wrap);
    chart = echarts.init(chartEl, "moonpool", { renderer: "canvas" });
    window.addEventListener("resize", function () { chart.resize(); });

    // ---- mapping persistence ----
    function storeKey() { return "mp:" + config.id + ":" + tabs[active].name; }
    function loadMapping() { try { return JSON.parse(localStorage.getItem(storeKey())); } catch (e) { return null; } }
    function saveMapping(m) { try { localStorage.setItem(storeKey(), JSON.stringify(m)); } catch (e) {} }

    // ---- render one tab ----
    function selectTab(i) { active = i; renderTabs(); renderTab(); }

    function renderTabs() {
      tabsEl.innerHTML = "";
      tabs.forEach(function (t, i) {
        var b = el("button", "tab" + (i === active ? " active" : ""), t.name);
        b.onclick = function () { selectTab(i); };
        tabsEl.appendChild(b);
      });
    }

    function renderTab() {
      var t = tabs[active];
      var profile = MP.profileRows(t.rows);
      var mapping = loadMapping() || t.mapping || MP.suggestMapping(profile);
      // sanity: drop mapping fields no longer present
      var names = {}; profile.forEach(function (c) { names[c.name] = c; });
      if (!names[mapping.x]) mapping.x = (profile[0] || {}).name;
      // X must suit the chart: scatter → numeric axis; others → a dimension
      var nums0 = profile.filter(function (c) { return c.type === "number"; });
      var dims0 = profile.filter(function (c) { return c.type !== "number"; });
      if (mapping.chart === "scatter") {
        if (!names[mapping.x] || names[mapping.x].type !== "number") mapping.x = (nums0[0] || profile[0] || {}).name;
      } else if (dims0.length && names[mapping.x] && names[mapping.x].type === "number") {
        mapping.x = dims0[0].name;
      }
      mapping.y = (mapping.y || []).filter(function (n) { return names[n]; });
      if (mapping.group && !names[mapping.group]) mapping.group = "";

      renderKpis(t.rows, profile);
      renderSchema(profile);
      renderMapStrip(profile, mapping);
      draw(t.rows, profile, mapping);
      renderTable(t, profile);
      renderCode(t);
    }

    function renderCode(t) {
      codeHead.textContent = "Raw " + (config.formatName || "source") + (t.file || t.source ? " · " + (t.source || t.file) : "");
      if (t.raw == null) {
        codeScroll.innerHTML = "<div class='code-empty'>Raw text not available for this dataset.</div>";
        return;
      }
      var text = String(t.raw), CAPB = 20000;
      var shown = text.length > CAPB ? text.slice(0, CAPB) : text;
      var pre = el("pre", "code-pre");
      pre.textContent = shown + (text.length > CAPB ? "\n… (" + (text.length - CAPB).toLocaleString() + " more chars)" : "");
      codeScroll.innerHTML = "";
      codeScroll.appendChild(pre);
    }

    var CAP = 200;
    function renderTable(t, profile) {
      var src = t.source || t.file;
      tableHead.textContent = "Source data" + (src ? " · " + src : "") +
        " – " + t.rows.length.toLocaleString() + " rows × " + profile.length + " cols";
      var cols = profile.map(function (c) { return c.name; });
      var rows = t.rows.slice(0, CAP);
      var h = "<table class='datatable'><thead><tr>";
      profile.forEach(function (c) { h += "<th class='ty-" + c.type + "'>" + esc(c.name) + "</th>"; });
      h += "</tr></thead><tbody>";
      rows.forEach(function (r) {
        h += "<tr>";
        profile.forEach(function (c) {
          var v = r[c.name];
          h += "<td class='ty-" + c.type + "'>" + (v == null || v === "" ? "<span class='na'>–</span>" : esc(v)) + "</td>";
        });
        h += "</tr>";
      });
      h += "</tbody></table>";
      if (t.rows.length > CAP) h += "<div class='table-more'>showing first " + CAP + " of " + t.rows.length.toLocaleString() + " rows</div>";
      tableScroll.innerHTML = h;
    }

    function renderKpis(rows, profile) {
      kpiRow.innerHTML = "";
      var tile = el("div", "card kpi");
      tile.appendChild(el("div", "label", "Records"));
      tile.appendChild(el("div", "value", rows.length.toLocaleString()));
      tile.appendChild(el("div", "delta", (profile.length) + " fields"));
      kpiRow.appendChild(tile);
      profile.filter(function (c) { return c.type === "number"; }).slice(0, 3).forEach(function (c) {
        var sum = 0; rows.forEach(function (r) { var v = MP.coerce(r[c.name], "number"); if (v != null) sum += v; });
        var k = el("div", "card kpi");
        k.appendChild(el("div", "label", "Σ " + c.name));
        k.appendChild(el("div", "value", fmt(sum)));
        k.appendChild(el("div", "delta", "avg " + fmt(sum / Math.max(1, rows.length))));
        kpiRow.appendChild(k);
      });
    }

    function renderSchema(profile) {
      schemaEl.innerHTML = "";
      profile.forEach(function (c) {
        var chip = el("span", "chip");
        chip.appendChild(el("span", "t t-" + c.type));
        chip.innerHTML += "<b>" + esc(c.name) + "</b> " + c.type + " · " + c.cardinality;
        schemaEl.appendChild(chip);
      });
    }

    function renderMapStrip(profile, mapping) {
      mapStrip.innerHTML = "";
      var nums = profile.filter(function (c) { return c.type === "number"; });
      var cats = profile.filter(function (c) { return c.type !== "number"; });

      // X – dimensions (date/category); scatter uses numeric axes instead
      var dims = profile.filter(function (c) { return c.type !== "number"; });
      var xCols = mapping.chart === "scatter" ? nums : (dims.length ? dims : profile);
      mapStrip.appendChild(fieldSelect("X axis", xCols.map(nm), mapping.x, function (v) {
        mapping.x = v; commit(mapping);
      }));

      // Measures (toggle chips)
      var mf = el("div", "field");
      mf.appendChild(el("label", null, "Measures"));
      var box = el("div", null); box.style.display = "flex"; box.style.gap = "6px"; box.style.flexWrap = "wrap";
      (nums.length ? nums : []).forEach(function (c) {
        var on = mapping.y.indexOf(c.name) >= 0;
        var b = el("button", "mtoggle" + (on ? " on" : ""), c.name);
        b.onclick = function () {
          var i = mapping.y.indexOf(c.name);
          if (i >= 0) mapping.y.splice(i, 1); else mapping.y.push(c.name);
          commit(mapping);
        };
        box.appendChild(b);
      });
      if (!nums.length) box.appendChild(el("span", "chip", "count of rows"));
      mf.appendChild(box); mapStrip.appendChild(mf);

      // Group by
      var groupOpts = [{ v: "", t: "– none –" }].concat(cats.map(function (c) { return { v: c.name, t: c.name }; }));
      mapStrip.appendChild(fieldSelectRaw("Group by", groupOpts, mapping.group, function (v) { mapping.group = v; commit(mapping); }));

      // Chart type
      mapStrip.appendChild(fieldSelectRaw("Chart", CHARTS, mapping.chart, function (v) { mapping.chart = v; commit(mapping); }));

      mapStrip.appendChild(el("div", "spacer"));

      // File button
      var ff = el("div", "field");
      ff.appendChild(el("label", null, "Your data"));
      var btn = el("label", "filebtn", "⬆ Load your " + (config.formatName || "file"));
      var input = el("input"); input.type = "file"; if (config.accept) input.accept = config.accept; input.style.display = "none";
      input.onchange = function () { if (input.files && input.files[0]) loadUserFile(input.files[0]); };
      btn.appendChild(input); ff.appendChild(btn); mapStrip.appendChild(ff);

      function commit(m) { saveMapping(m); renderTab(); }
    }

    function nm(c) { return { v: c.name, t: c.name }; }
    function fieldSelect(label, opts, val, on) { return fieldSelectRaw(label, opts, val, on); }
    function fieldSelectRaw(label, opts, val, on) {
      var f = el("div", "field");
      f.appendChild(el("label", null, label));
      var s = el("select");
      opts.forEach(function (o) {
        var op = el("option"); op.value = o.v; op.textContent = o.t; if (o.v === val) op.selected = true; s.appendChild(op);
      });
      s.onchange = function () { on(s.value); };
      f.appendChild(s); return f;
    }

    // ---- chart building ----
    function draw(rows, profile, mapping) {
      var byName = {}; profile.forEach(function (c) { byName[c.name] = c; });
      var xCol = byName[mapping.x];
      chartTitle.textContent = titleFor(mapping, xCol);
      if (!xCol) { chart.clear(); return; }

      if (mapping.chart === "scatter") return drawScatter(rows, byName, mapping);
      if (mapping.chart === "pie")     return drawPie(rows, byName, mapping);
      drawCartesian(rows, byName, mapping);   // line / area / bar
    }

    function titleFor(m, xCol) {
      var meas = m.y.length ? m.y.join(", ") : "count";
      return meas + " by " + (m.x || "?") + (m.group ? " · split by " + m.group : "");
    }

    // Aggregate sum(measure) [or count] over x [and group].
    function aggregate(rows, xCol, measures, groupCol) {
      var xType = xCol.type;
      var acc = {};            // xKey -> { series -> sum }
      var order = [];          // xKeys in first-seen order
      var groups = {};
      rows.forEach(function (r) {
        var xv = MP.coerce(r[xCol.name], xType);
        if (xv == null) return;
        var xKey = String(xv);
        if (!(xKey in acc)) { acc[xKey] = {}; order.push({ key: xKey, val: xv }); }
        var g = groupCol ? String(MP.coerce(r[groupCol.name], "category")) : null;
        function add(series, amount) { groups[series] = 1; acc[xKey][series] = (acc[xKey][series] || 0) + amount; }
        if (measures.length) {
          measures.forEach(function (mName) {
            var v = MP.coerce(r[mName], "number"); if (v == null) return;
            add(groupCol ? (mName + " · " + g) : mName, v);
          });
        } else {
          add(groupCol ? g : "count", 1);
        }
      });
      // sort x
      if (xType === "date" || xType === "number") order.sort(function (a, b) { return a.val - b.val; });
      var cats = order.map(function (o) { return xType === "date" ? dateLabel(o.val) : o.key; });
      var seriesNames = Object.keys(groups);
      var series = seriesNames.map(function (sn) {
        return { name: sn, data: order.map(function (o) { var v = acc[o.key][sn]; return v == null ? null : Math.round(v * 1000) / 1000; }) };
      });
      return { cats: cats, series: series };
    }

    function drawCartesian(rows, byName, m) {
      var measures = m.y.filter(function (n) { return byName[n] && byName[n].type === "number"; });
      var groupCol = m.group ? byName[m.group] : null;
      var agg = aggregate(rows, byName[m.x], measures, groupCol);
      var area = m.chart === "area";
      chart.setOption({
        tooltip: { trigger: "axis" },
        legend: { show: agg.series.length > 1, top: 0, type: "scroll" },
        xAxis: { type: "category", data: agg.cats, boundaryGap: m.chart === "bar", axisLabel: { hideOverlap: true } },
        yAxis: { type: "value" },
        dataZoom: agg.cats.length > 40 ? [{ type: "inside" }, { type: "slider", height: 18, bottom: 8 }] : [],
        series: agg.series.map(function (s) {
          return {
            name: s.name, type: m.chart === "bar" ? "bar" : "line", data: s.data,
            stack: (m.chart === "bar" && agg.series.length > 1) ? "s" : (area ? "s" : null),
            areaStyle: area ? { opacity: 0.25 } : null,
            smooth: !area, showSymbol: agg.cats.length <= 60, connectNulls: true
          };
        })
      }, true);
    }

    function drawScatter(rows, byName, m) {
      var nums = Object.keys(byName).filter(function (k) { return byName[k].type === "number"; });
      var xn = byName[m.x] && byName[m.x].type === "number" ? m.x : nums[0];
      var yn = m.y[0] && byName[m.y[0]] ? m.y[0] : nums.find(function (n) { return n !== xn; }) || nums[0];
      var groupCol = m.group ? byName[m.group] : null;
      var buckets = {};
      rows.forEach(function (r) {
        var x = MP.coerce(r[xn], "number"), y = MP.coerce(r[yn], "number");
        if (x == null || y == null) return;
        var g = groupCol ? String(MP.coerce(r[groupCol.name], "category")) : yn;
        (buckets[g] = buckets[g] || []).push([x, y]);
      });
      chart.setOption({
        tooltip: { trigger: "item", formatter: function (p) { return xn + ": " + fmt(p.value[0]) + "<br>" + yn + ": " + fmt(p.value[1]); } },
        legend: { show: Object.keys(buckets).length > 1, top: 0, type: "scroll" },
        xAxis: { type: "value", name: xn, scale: true },
        yAxis: { type: "value", name: yn, scale: true },
        series: Object.keys(buckets).map(function (g) { return { name: g, type: "scatter", data: buckets[g] }; })
      }, true);
    }

    function drawPie(rows, byName, m) {
      var xCol = byName[m.x];
      var measure = m.y.filter(function (n) { return byName[n] && byName[n].type === "number"; })[0];
      var sums = {}, order = [];
      rows.forEach(function (r) {
        var k = MP.coerce(r[xCol.name], "category"); if (k == null) return;
        if (!(k in sums)) { sums[k] = 0; order.push(k); }
        sums[k] += measure ? (MP.coerce(r[measure], "number") || 0) : 1;
      });
      var data = order.map(function (k) { return { name: k, value: Math.round(sums[k] * 1000) / 1000 }; })
                      .sort(function (a, b) { return b.value - a.value; }).slice(0, 12);
      var total = data.reduce(function (s, d) { return s + d.value; }, 0);
      chart.setOption({
        tooltip: { trigger: "item", formatter: "{b}<br><b>{c}</b> ({d}%)" },
        legend: { show: true, type: "scroll", orient: "vertical", right: 12, top: "middle", itemGap: 10 },
        title: {
          text: fmt(total), subtext: (measure || "total").toUpperCase(),
          left: "39%", top: "43%", textAlign: "center",
          textStyle: { color: "#e6e9f0", fontSize: 30, fontWeight: 650 },
          subtextStyle: { color: "#9aa4b8", fontSize: 11, letterSpacing: 1 }
        },
        series: [{
          type: "pie", radius: ["55%", "78%"], center: ["40%", "52%"], avoidLabelOverlap: true,
          itemStyle: { borderRadius: 6, borderColor: "#131722", borderWidth: 3 },
          label: { show: true, color: "#9aa4b8", formatter: "{b}  {d}%", fontSize: 12 },
          labelLine: { length: 12, length2: 12, lineStyle: { color: "#2a3042" } },
          emphasis: { scale: true, scaleSize: 8, label: { color: "#e6e9f0", fontWeight: 600 },
                      itemStyle: { shadowBlur: 18, shadowColor: "#0008" } },
          data: data
        }]
      }, true);
    }

    // ---- user file loading ----
    // Load the user's file into the dedicated "Your data" tab (samples preserved).
    function loadUserFile(file) {
      chartTitle.textContent = "Parsing " + file.name + "…";
      // read raw text in parallel for the code pane (best-effort; skip if unreadable)
      var rawP = (file.text ? file.text() : new Promise(function (res) {
        var fr = new FileReader(); fr.onload = function () { res(fr.result); }; fr.onerror = function () { res(null); }; fr.readAsText(file);
      })).catch(function () { return null; });
      Promise.all([Promise.resolve(config.parseFile(file)), rawP]).then(function (out) {
        var rows = out[0], rawText = out[1];
        if (!rows || !rows.length) throw new Error("no rows parsed");
        var name = "Your data";
        var existing = tabs.findIndex(function (t) { return t.name === name; });
        var tab = { name: name, rows: rows, source: file.name, raw: rawText };
        if (existing >= 0) tabs[existing] = tab; else tabs.push(tab);
        try { localStorage.removeItem("mp:" + config.id + ":" + name); } catch (e) {}  // fresh auto-profile
        selectTab(tabs.findIndex(function (t) { return t.name === name; }));
      }).catch(function (err) {
        chartTitle.textContent = "Could not read that file: " + (err && err.message || err);
      });
    }

    // drag-drop (works in a real browser; Tauri webview may swallow it – button is primary)
    ["dragenter", "dragover"].forEach(function (ev) {
      chartCard.addEventListener(ev, function (e) { e.preventDefault(); chartCard.classList.add("drag"); });
    });
    ["dragleave", "drop"].forEach(function (ev) {
      chartCard.addEventListener(ev, function (e) { e.preventDefault(); if (ev === "dragleave" && chartCard.contains(e.relatedTarget)) return; chartCard.classList.remove("drag"); });
    });
    chartCard.addEventListener("drop", function (e) {
      var f = e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0]; if (f) loadUserFile(f);
    });

    function esc(s) { return String(s).replace(/[&<>]/g, function (c) { return { "&": "&amp;", "<": "&lt;", ">": "&gt;" }[c]; }); }

    // go
    renderTabs(); renderTab();
    return { reload: renderTab };
  };
})();
