import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import AddTransactionModal from "./AddTransactionModal";
import HoldingsSidebar from "./HoldingsSidebar";

interface PortfolioHolding {
  ticker: string;
  quantity: number;
  avg_cost: number;
  market_price?: number;
  market_value?: number;
  unrealized_pl?: number;
}

interface PortfolioSummary {
  holdings: PortfolioHolding[];
  cash_balance: number;
  total_value: number;
  total_unrealized_pl: number;
  total_realized_pl: number;
}

const PortfolioPage: React.FC<{ portfolioId: number; userId: number }> = ({ portfolioId, userId }) => {
  const [summary, setSummary] = useState<PortfolioSummary | null>(null);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [sidebarKey, setSidebarKey] = useState(0);

  useEffect(() => {
    setLoading(true);
    invoke("get_portfolio_summary", { portfolio_id: portfolioId, user_id: userId })
      .then((result: any) => {
        setSummary(typeof result === 'string' ? JSON.parse(result) : result);
      })
      .finally(() => setLoading(false));
  }, [portfolioId, userId]);

  const handleTransactionAdded = () => {
    // Refresca el resumen y la barra de holdings
    setLoading(true);
    invoke("get_portfolio_summary", { portfolio_id: portfolioId, user_id: userId })
      .then((result: any) => {
        setSummary(typeof result === 'string' ? JSON.parse(result) : result);
        setSidebarKey(k => k + 1); // fuerza el refresh del sidebar
      })
      .finally(() => setLoading(false));
  };

  return (
    <div className="portfolio-page">
      <h1>Mi Portafolio</h1>
      <button onClick={() => setShowModal(true)}>Añadir Transacción</button>
      <div style={{display:'flex',gap:'2rem'}}>
        <HoldingsSidebar key={sidebarKey} portafolioId={portfolioId} />
        <div style={{flex:1}}>
          {loading && <div>Cargando...</div>}
          {summary && (
            <>
              <div className="portfolio-summary">
                <div>Efectivo: ${summary.cash_balance.toLocaleString('es-MX')}</div>
                <div>Valor Total: ${summary.total_value.toLocaleString('es-MX')}</div>
                <div>Rendimiento No Realizado: ${summary.total_unrealized_pl.toLocaleString('es-MX')}</div>
                <div>Rendimiento Realizado: ${summary.total_realized_pl.toLocaleString('es-MX')}</div>
              </div>
              <table className="portfolio-holdings-table">
                <thead>
                  <tr>
                    <th>Ticker</th>
                    <th>Cantidad</th>
                    <th>Costo Promedio</th>
                    <th>Precio Mercado</th>
                    <th>Valor Mercado</th>
                    <th>G/P No Realizado</th>
                  </tr>
                </thead>
                <tbody>
                  {summary.holdings.map(h => (
                    <tr key={h.ticker}>
                      <td>{h.ticker}</td>
                      <td>{h.quantity}</td>
                      <td>${h.avg_cost.toLocaleString('es-MX')}</td>
                      <td>{h.market_price !== undefined ? `$${h.market_price.toLocaleString('es-MX')}` : '-'}</td>
                      <td>{h.market_value !== undefined ? `$${h.market_value.toLocaleString('es-MX')}` : '-'}</td>
                      <td style={{color: h.unrealized_pl && h.unrealized_pl > 0 ? 'var(--color-positive)' : 'var(--color-negative)'}}>
                        {h.unrealized_pl !== undefined ? `$${h.unrealized_pl.toLocaleString('es-MX')}` : '-'}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </>
          )}
        </div>
      </div>
      <AddTransactionModal open={showModal} onClose={() => setShowModal(false)} portfolioId={portfolioId} userId={userId} onTransactionAdded={handleTransactionAdded} />
    </div>
  );
};

export default PortfolioPage;
