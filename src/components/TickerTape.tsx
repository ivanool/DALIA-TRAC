import React, { useRef, useState, useEffect } from "react";
import "./TickerTape.css";

interface TickerTapeProps {
  indices: any;
  forex: any;
  top: any;
  symbols?: { proName: string; title: string }[];
}

const defaultSymbols = [
  { proName: "FOREXCOM:SPXUSD", title: "S&P 500" },
  { proName: "FOREXCOM:NSXUSD", title: "Nasdaq 100" },
  { proName: "FX_IDC:EURUSD", title: "EUR to USD" },
  { proName: "BITSTAMP:BTCUSD", title: "Bitcoin" },
  { proName: "BITSTAMP:ETHUSD", title: "Ethereum" }
];

const TickerTape: React.FC<TickerTapeProps> = ({ indices, forex, top, symbols = [] }) => {
  const items: { symbol: string, value: number | string, change: number, type: string }[] = [];
  if (indices) {
    Object.entries(indices).forEach(([k, v]: any) => v && items.push({ symbol: k, value: v.u, change: v.c, type: 'Índice' }));
  }
  if (forex) {
    if (forex.USDMXN) items.push({ symbol: 'USDMXN', value: forex.USDMXN.u, change: forex.USDMXN.c, type: 'Divisa' });
    if (forex.EURMXN) items.push({ symbol: 'EURMXN', value: forex.EURMXN.u, change: forex.EURMXN.c, type: 'Divisa' });
  }
  if (top && top.suben) {
    top.suben.slice(0, 5).forEach((t: any) => {
      items.push({ symbol: t.e, value: t.u, change: t.c, type: 'Sube' });
    });
  }
  if (top && top.bajan) {
    top.bajan.slice(0, 5).forEach((t: any) => {
      items.push({ symbol: t.e, value: t.u, change: t.c, type: 'Baja' });
    });
  }
  const [minWidth, setMinWidth] = useState(0);
  const [pauseAt, setPauseAt] = useState<number | null>(null);
  const tapeRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [tooltip, setTooltip] = useState<{ x: number; y: number; text: string } | null>(null);
  const [glow, setGlow] = useState(false);
  const [reboundIndex, setReboundIndex] = useState<number | null>(null);

  useEffect(() => {
    if (tapeRef.current) {
      setMinWidth(tapeRef.current.offsetWidth);
    }
  }, [indices, forex, top]);

  let repeat = 2;
  if (typeof window !== 'undefined' && minWidth > 0) {
    const vw = window.innerWidth;
    repeat = Math.ceil((vw * 2.5) / minWidth);
  }
  const tapeItems = Array(repeat).fill(items).flat();

  useEffect(() => {
    if (tapeRef.current) {
      tapeRef.current.style.animation = 'none';
      void tapeRef.current.offsetWidth;
      tapeRef.current.style.animation = '';
    }
  }, [tapeItems.length]);

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!tapeRef.current || !containerRef.current) return;
    const containerRect = containerRef.current.getBoundingClientRect();
    const mouseX = e.clientX - containerRect.left;
    const percent = mouseX / containerRect.width;
    const totalWidth = tapeRef.current.scrollWidth;
    const offset = percent * (totalWidth - containerRect.width);
    tapeRef.current.style.animationPlayState = 'paused';
    tapeRef.current.style.transform = `translateX(-${offset}px)`;
    setPauseAt(offset);
  };
  const handleMouseLeave = () => {
    if (tapeRef.current) {
      tapeRef.current.style.animationPlayState = '';
      tapeRef.current.style.transform = '';
    }
    setPauseAt(null);
  };

  const handleItemMouseEnter = (e: React.MouseEvent, t: any, i: number) => {
    const rect = (e.target as HTMLElement).getBoundingClientRect();
    setTooltip({
      x: rect.left + rect.width / 2,
      y: rect.top,
      text: `${t.symbol} | ${t.change >= 0 ? '+' : ''}${t.change}%`,
    });
    setGlow(true);
    setReboundIndex(i);
  };
  const handleItemMouseLeave = () => {
    setTooltip(null);
    setGlow(false);
    setReboundIndex(null);
  };

  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.innerHTML = "";
      const script = document.createElement("script");
      script.src = "https://s3.tradingview.com/external-embedding/embed-widget-ticker-tape.js";
      script.async = true;
      script.innerHTML = JSON.stringify({
        symbols: symbols.length > 0 ? symbols : defaultSymbols,
        showSymbolLogo: true,
        colorTheme: "dark",
        isTransparent: true,
        displayMode: "adaptive",
        locale: "es"
      });
      containerRef.current.appendChild(script);
    }
  }, [symbols]);

  return (
    <>
      <div
        className={`ticker-tape-minimal${glow ? ' ticker-glow' : ''}`}
        ref={containerRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
      >
        <div
          className="ticker-tape-inner"
          ref={tapeRef}
          style={pauseAt !== null ? { animationPlayState: 'paused' } : {}}
        >
          {tapeItems.map((t, i) => (
            <span
              className={`ticker-item${reboundIndex === i ? ' ticker-rebound' : ''}`}
              key={i}
              onMouseEnter={e => handleItemMouseEnter(e, t, i)}
              onMouseLeave={handleItemMouseLeave}
            >
              <span className="ticker-symbol">{t.symbol}</span>
              <span>{t.value !== undefined ? t.value : '-'}</span>
              <span className={t.change >= 0 ? "ticker-up" : "ticker-down"}>
                {t.change >= 0 ? (
                  <>
                    <span style={{color:'#4caf50',fontWeight:700}}>▲</span> +{t.change}%
                  </>
                ) : (
                  <>
                    <span style={{color:'#ff5722',fontWeight:700}}>▼</span> {t.change}%
                  </>
                )}
              </span>
              <span className="ticker-separator">·</span>
            </span>
          ))}
        </div>
        {tooltip && (
          <div
            className="ticker-tooltip"
            style={{ left: tooltip.x, top: tooltip.y - 32 }}
          >
            {tooltip.text}
          </div>
        )}
      </div>
      <div className="tradingview-widget-container" ref={containerRef}>
        <div className="tradingview-widget-container__widget"></div>
      </div>
    </>
  );
};

export default TickerTape;
