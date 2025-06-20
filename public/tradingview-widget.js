// TradingView Widget Loader
export function loadTradingViewWidget(containerId, symbol) {
  if (!window.TradingView) {
    const script = document.createElement('script');
    script.src = 'https://s3.tradingview.com/tv.js';
    script.onload = () => renderWidget();
    document.body.appendChild(script);
  } else {
    renderWidget();
  }
  function renderWidget() {
    new window.TradingView.widget({
      autosize: true,
      symbol: symbol,
      interval: 'D',
      timezone: 'Etc/UTC',
      theme: 'light',
      style: '1',
      locale: 'es',
      container_id: containerId,
      enable_publishing: false,
      hide_top_toolbar: false,
      hide_legend: false,
      allow_symbol_change: false,
      details: true,
      hotlist: false,
      calendar: false,
      studies: [],
    });
  }
}
