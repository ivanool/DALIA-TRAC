import React, { useEffect, useState } from "react";
import "./HoldingsSidebar.css";
import { invoke } from "@tauri-apps/api/core";

interface HoldingsSidebarProps {
  portafolioId: number;
}

const HoldingsSidebar: React.FC<HoldingsSidebarProps> = ({ portafolioId }) => {
  const [holdings, setHoldings] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    invoke('get_holdings_by_portafolio', { portafolio_id: portafolioId })
      .then((data: any) => {
        setHoldings(data);
        setError(null);
      })
      .catch(() => {
        setError('No se pudieron cargar los holdings');
        setHoldings([]);
      })
      .finally(() => setLoading(false));
  }, [portafolioId]);

  if (loading) return <div className="holdings-placeholder">Cargando holdings...</div>;
  if (error) return <div className="holdings-placeholder">{error}</div>;
  if (!holdings.length) return <div className="holdings-placeholder">No tienes holdings en este portafolio.</div>;

  return (
    <ul className="holdings-list">
      {holdings.map((h, i) => (
        <li key={i} className="holding-item">
          <b>{h.ticker}</b> <span style={{color:'#888'}}>{h.emisoras}{h.serie ? '.'+h.serie : ''}</span>
          <span style={{float:'right',fontWeight:600}}>{h.cantidad}</span>
        </li>
      ))}
    </ul>
  );
};

export default HoldingsSidebar;
