import { useEffect, useRef } from "react";
import { createChart, IChartApi, LineData } from "lightweight-charts";

interface Props {
  symbol: string;
  data: LineData[];
}

export default function LightweightChart({ symbol, data }: Props) {
  const chartRef = useRef<HTMLDivElement>(null);
  const chartInstance = useRef<IChartApi | null>(null);

  useEffect(() => {
    if (!chartRef.current) return;
    if (chartInstance.current) {
      chartInstance.current.remove();
    }
    const chart = createChart(chartRef.current, {
      width: chartRef.current.offsetWidth,
      height: 320,
      layout: { background: { color: '#f6f6f6' }, textColor: '#212121' },
      grid: { vertLines: { color: '#eee' }, horzLines: { color: '#eee' } },
      timeScale: { timeVisible: true, secondsVisible: false },
    });
    chartInstance.current = chart;
    const series = chart.addLineSeries();
    series.setData(data);
    return () => chart.remove();
  }, [symbol, data]);

  return <div ref={chartRef} style={{ width: '100%', height: 320 }} />;
}
