/* Registers the "moonpool" ECharts theme. Load after echarts.min.js, before use.
   Mirrors the CSS tokens in theme.css so charts read as one system. */
(function () {
  if (typeof echarts === "undefined") { console.warn("echarts not loaded before theme"); return; }
  var palette = ["#5b8cff","#3ecf8e","#f5a623","#b57bff","#ff7a8a","#35c4d4","#f06fb0","#c3d34b"];
  var fg = "#e6e9f0", muted = "#9aa4b8", line = "#2a3042", dim = "#6b7488";
  var axis = {
    axisLine:  { lineStyle: { color: line } },
    axisTick:  { lineStyle: { color: line } },
    axisLabel: { color: muted },
    splitLine: { lineStyle: { color: "#20263580" } },
    splitArea: { areaStyle: { color: ["#ffffff03", "#ffffff00"] } },
    nameTextStyle: { color: dim }
  };
  echarts.registerTheme("moonpool", {
    color: palette,
    backgroundColor: "transparent",
    textStyle: { fontFamily: '-apple-system, "Segoe UI", Roboto, sans-serif', color: fg },
    title: { textStyle: { color: fg }, subtextStyle: { color: muted } },
    legend: { textStyle: { color: muted }, inactiveColor: "#3a4152", icon: "roundRect" },
    grid: { left: 56, right: 28, top: 48, bottom: 44, containLabel: true },
    tooltip: {
      backgroundColor: "#1a1f2ef2", borderColor: line, borderWidth: 1,
      textStyle: { color: fg }, padding: [8, 12],
      axisPointer: { lineStyle: { color: line }, crossStyle: { color: line }, label: { backgroundColor: "#212637" } }
    },
    categoryAxis: axis, valueAxis: axis, logAxis: axis, timeAxis: axis,
    line: { symbol: "circle", symbolSize: 6, lineStyle: { width: 2.5 }, smooth: true },
    bar: { itemStyle: { borderRadius: [4, 4, 0, 0] } },
    pie: { itemStyle: { borderColor: "#131722", borderWidth: 2 } },
    scatter: { symbolSize: 9 },
    visualMap: { textStyle: { color: muted } },
    dataZoom: {
      textStyle: { color: dim },
      handleStyle: { color: "#5b8cff" }, borderColor: line,
      dataBackground: { lineStyle: { color: line }, areaStyle: { color: "#5b8cff22" } },
      fillerColor: "#5b8cff1a"
    }
  });
})();
