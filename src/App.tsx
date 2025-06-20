import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import LightweightChart from "./LightweightChart";

const HomeIcon = () => (
  <svg className="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M3 9l9-7 9 7"/><path d="M9 22V12H15V22"/><path d="M21 22H3"/></svg>
);
const ChartIcon = () => (
  <svg className="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="12" width="6" height="8"/><rect x="9" y="8" width="6" height="12"/><rect x="15" y="4" width="6" height="16"/></svg>
);
const UserIcon = () => (
  <svg className="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="7" r="4"/><path d="M5.5 21a8.38 8.38 0 0 1 13 0"/></svg>
);
const SettingsIcon = () => (
  <svg className="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09a1.65 1.65 0 0 0-1-1.51 1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09a1.65 1.65 0 0 0 1.51-1 1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33h.09A1.65 1.65 0 0 0 9 3.09V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51h.09a1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82v.09a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>
);

const DaliaFlower = () => (
  <svg className="dalia-flower" viewBox="0 0 32 32" fill="none">
    <circle cx="16" cy="16" r="6" stroke="#181A20" strokeWidth="2" fill="#fff"/>
    <g stroke="#181A20" strokeWidth="2">
      <ellipse cx="16" cy="5" rx="2" ry="5"/>
      <ellipse cx="16" cy="27" rx="2" ry="5"/>
      <ellipse cx="5" cy="16" rx="5" ry="2"/>
      <ellipse cx="27" cy="16" rx="5" ry="2"/>
      <ellipse cx="8.5" cy="8.5" rx="2" ry="5" transform="rotate(-45 8.5 8.5)"/>
      <ellipse cx="23.5" cy="23.5" rx="2" ry="5" transform="rotate(-45 23.5 23.5)"/>
      <ellipse cx="8.5" cy="23.5" rx="2" ry="5" transform="rotate(45 8.5 23.5)"/>
      <ellipse cx="23.5" cy="8.5" rx="2" ry="5" transform="rotate(45 23.5 8.5)"/>
    </g>
  </svg>
);

// Usa la flor Dalia como logo animado en la barra superior
const DaliaLogo = DaliaFlower;

function TickerTape({ indices, forex, top }: { indices: any, forex: any, top: any }) {
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
  // Para evitar espacios en blanco, duplicar los items hasta que llenen al menos 2.5 veces el ancho de la pantalla
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

  // Duplicar los items hasta cubrir suficiente ancho
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

  // Pausar animación exactamente donde está el mouse
  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!tapeRef.current || !containerRef.current) return;
    const containerRect = containerRef.current.getBoundingClientRect();
    const mouseX = e.clientX - containerRect.left;
    // Calcula el porcentaje del mouse en el contenedor
    const percent = mouseX / containerRect.width;
    // Calcula el offset actual de la animación
    const totalWidth = tapeRef.current.scrollWidth;
    const offset = percent * (totalWidth - containerRect.width);
    // Aplica el transform para pausar justo ahí
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

  // Tooltip y rebote
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

  return (
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
  );
}

// Hook para cachear datos temporalmente en memoria
function useCachedData<T>(fetchFn: () => Promise<T>, intervalMs: number): [T | null, boolean] {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const lastFetch = useRef<number>(0);

  useEffect(() => {
    let mounted = true;
    async function fetchData() {
      setLoading(true);
      try {
        const now = Date.now();
        if (!data || now - lastFetch.current > intervalMs) {
          const result = await fetchFn();
          if (mounted) {
            setData(result);
            lastFetch.current = now;
          }
        }
      } finally {
        if (mounted) setLoading(false);
      }
    }
    fetchData();
    const timer = setInterval(fetchData, intervalMs);
    return () => { mounted = false; clearInterval(timer); };
    // eslint-disable-next-line
  }, []);
  return [data, loading];
}

// Modo oscuro manual
function useDarkMode() {
  const [dark, setDark] = useState(false);
  useEffect(() => {
    document.body.classList.toggle('dark-mode', dark);
  }, [dark]);
  return [dark, setDark] as [boolean, (v: boolean) => void];
}

function App() {
  const [info, setInfo] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [search, setSearch] = useState("");
  const [results, setResults] = useState<EmisoraBusqueda[]>([]);
  const [selectedEmpresa, setSelectedEmpresa] = useState<EmisoraBusqueda | null>(null);
  const tradingViewRef = useRef<HTMLDivElement>(null);
  // Usar cache de 20 minutos (1200000 ms)
  const [indices, indicesLoading] = useCachedData(() => invoke("get_indices_tauri"), 1200000);
  const [forex, forexLoading] = useCachedData(() => invoke("get_forex_tauri"), 1200000);
  const [top, topLoading] = useCachedData(() => invoke("get_top_tauri"), 1200000);
  const [chartData, setChartData] = useState<any[]>([]);
  const [dark, setDark] = useDarkMode();

  useEffect(() => {
    if (search.length < 2) {
      setResults([]);
      return;
    }
    const timeout = setTimeout(() => {
      invoke<EmisoraBusqueda[]>("buscar_emisoras", { query: search })
        .then(setResults)
        .catch(() => setResults([]));
    }, 300);
    return () => clearTimeout(timeout);
  }, [search]);

  useEffect(() => {
    if (selectedEmpresa) {
      const now = Math.floor(Date.now() / 1000);
      const data = Array.from({ length: 60 }, (_, i) => ({
        time: now - (60 - i) * 86400,
        value: Math.round(100 + Math.random() * 20),
      }));
      setChartData(data);
    }
  }, [selectedEmpresa]);

  const [now, setNow] = useState(new Date());
  useEffect(() => {
    const interval = setInterval(() => setNow(new Date()), 60000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="app-layout">
      {/* Switch modo oscuro */}
      <button
        className="dark-toggle"
        onClick={() => setDark(!dark)}
        title={dark ? 'Modo claro' : 'Modo oscuro'}
      >
        {dark ? '🌙' : '☀️'}
      </button>
      {/* Línea superior: ticker tape */}
      <TickerTape indices={indices} forex={forex} top={top} />
      {/* Barra de búsqueda y opciones */}
      <header className="top-bar minimal">
        <div className="logo-title">
          <DaliaLogo />
        </div>
        <div style={{ position: "relative", flex: 1 }}>
          <input
            className="search-bar"
            placeholder="Buscar empresa o ticker..."
            value={search}
            onChange={e => {
              setSearch(e.target.value);
              setSelectedEmpresa(null);
            }}
            autoComplete="off"
          />
          {results.length > 0 && (
            <ul className="search-dropdown">
              {results.map((r, i) => (
                <li key={i} onClick={() => {
                  setSelectedEmpresa(r);
                  setSearch(`${r.razon_social} (${r.emisoras}${r.serie ? "." + r.serie : ""})`);
                  setResults([]);
                }}>
                  <b>{r.emisoras}{r.serie ? "." + r.serie : ""}</b> — {r.razon_social}
                </li>
              ))}
            </ul>
          )}
        </div>
      </header>
      {/* Layout principal dividido en 3/4 y 1/4 */}
      <div className="main-content minimal">
        {/*
        <nav className="sidebar">
        </nav>
        */}
        <main className="presentation-sheet">
          {/* Mostrar datos de mercado solo si hay datos, si no, mostrar mensaje centrado */}
          {(indices && Object.keys(indices).length > 0) || (forex && (forex.USDMXN || forex.EURMXN)) || (top && Array.isArray(top.importe) && top.importe.length > 0) ? (
            <div className="presentation-placeholder">
              <h2>Datos de Mercado</h2>
              <div className="market-data-blocks">
                {/* Indices */}
                {indices && Object.keys(indices).length > 0 && (
                  <div className="market-block">
                    <h3>Índices</h3>
                    <ul>
                      {Object.entries(indices).map(([k, v]: any) => v && (
                        <li key={k}><b>{k}</b>: {v.u} ({v.c >= 0 ? '+' : ''}{v.c}%)</li>
                      ))}
                    </ul>
                  </div>
                )}
                {/* Divisas */}
                {forex && (forex.USDMXN || forex.EURMXN) && (
                  <div className="market-block">
                    <h3>Divisas</h3>
                    <ul>
                      {forex.USDMXN && <li><b>USDMXN</b>: {forex.USDMXN.u} ({forex.USDMXN.c >= 0 ? '+' : ''}{forex.USDMXN.c}%)</li>}
                      {forex.EURMXN && <li><b>EURMXN</b>: {forex.EURMXN.u} ({forex.EURMXN.c >= 0 ? '+' : ''}{forex.EURMXN.c}%)</li>}
                    </ul>
                  </div>
                )}
                {/* Top */}
                {top && Array.isArray(top.importe) && top.importe.length > 0 && (
                  <div className="market-block">
                    <h3>Top Importe</h3>
                    <ul>
                      {top.importe.slice(0, 5).map((item: any, idx: number) => (
                        <li key={idx}><b>{item.e}</b>: {item.u}</li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className="presentation-empty">
              <span>Sin datos de mercado disponibles.</span>
            </div>
          )}
        </main>
        <aside className="holdings-section minimal">
          <h2>Holdings</h2>
          <div className="holdings-placeholder">
            <span>Próximamente: Tus holdings aparecerán aquí.</span>
          </div>
        </aside>
      </div>
    </div>
  );
}

export default App;
