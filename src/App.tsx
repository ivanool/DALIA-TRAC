import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import TickerTape from "./components/TickerTape";
import HoldingsSidebar from "./components/HoldingsSidebar";
import EmisoraPage from "./components/EmisoraPage";
import { HomeIcon, ChartIcon, UserIcon, DaliaFlower } from "./components/Icons";
import { EmisoraBusqueda, IndicesType, ForexType, TopType } from "./types";
import useCachedData from "./hooks/useCachedData";

const DaliaLogo = DaliaFlower;

function App() {
  const [search, setSearch] = useState("");
  const [results, setResults] = useState<EmisoraBusqueda[]>([]);
  const [selectedEmpresa, setSelectedEmpresa] = useState<EmisoraBusqueda | null>(null);
  const [activeMenu, setActiveMenu] = useState('portafolios');
  const [indices] = useCachedData(() => invoke("get_indices_tauri"), 1200000) as [IndicesType | null, boolean];
  const [forex] = useCachedData(() => invoke("get_forex_tauri"), 1200000) as [ForexType | null, boolean];
  const [top] = useCachedData(() => invoke("get_top_tauri"), 1200000) as [TopType | null, boolean];
  const [page, setPage] = useState<'main' | 'emisora'>("main");
  const [emisoraActual, setEmisoraActual] = useState<EmisoraBusqueda | null>(null);

  useEffect(() => {
    if (search.length < 2) {
      setResults([]);
      return;
    }
    const timeout = setTimeout(() => {
      invoke("buscar_emisoras", { query: search })
        .then((res: any) => {
          if (Array.isArray(res)) {
            setResults(res);
          } else {
            setResults([]);
          }
        })
        .catch(() => setResults([]));
    }, 300);
    return () => clearTimeout(timeout);
  }, [search]);

  return (
    <div className="app-root-layout">
      {/* Barra superior: logo, búsqueda, opciones */}
      <header className="top-bar redesign">
        <div className="logo-title">
          <DaliaLogo />
        </div>
        <div className="search-bar-container">
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
                  setEmisoraActual(r);
                  setPage('emisora');
                  setResults([]);
                }}>
                  <b>{r.emisoras}{r.serie ? "." + r.serie : ""}</b> — {r.razon_social}
                </li>
              ))}
            </ul>
          )}
        </div>
        <nav className="main-menu-options right-align">
          <button className={activeMenu === 'portafolios' ? 'active' : ''} onClick={() => setActiveMenu('portafolios')}><HomeIcon /> Portafolios</button>
          <button className={activeMenu === 'transacciones' ? 'active' : ''} onClick={() => setActiveMenu('transacciones')}><ChartIcon /> Transacciones</button>
          <button className={activeMenu === 'fundamentos' ? 'active' : ''} onClick={() => setActiveMenu('fundamentos')}><UserIcon /> Fundamentos</button>
        </nav>
      </header>
      {/* Ticker tape justo debajo de la barra superior */}
      <div className="ticker-tape-bar">
        <TickerTape indices={indices} forex={forex} top={top} />
      </div>
      {/* Layout principal: panel grande y sidebar de holdings */}
      <div className="main-content-layout grid-layout">
        <main className="main-panel-content">
          {page === 'emisora' && emisoraActual ? (
            <EmisoraPage emisora={emisoraActual} onBack={() => setPage('main')} />
          ) : (
            <div className="presentation-sheet">
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
            </div>
          )}
        </main>
        <aside className="holdings-section minimal sidebar-fixed">
          <h2>Holdings</h2>
          <HoldingsSidebar portafolioId={1} />
        </aside>
      </div>
    </div>
  );
}

export default App;
