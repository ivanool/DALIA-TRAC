import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Portfolio {
  id: number;
  nombre: string;
  id_hex: string;
}

interface PortfolioSelectorPageProps {
  userId: number;
  onPortfolioSelected: (portfolio: Portfolio) => void;
}

const PortfolioSelectorPage: React.FC<PortfolioSelectorPageProps> = ({ userId, onPortfolioSelected }) => {
  const [portfolios, setPortfolios] = useState<Portfolio[]>([]);
  const [loading, setLoading] = useState(true);
  const [newPortfolioName, setNewPortfolioName] = useState('');
  const [error, setError] = useState<string | null>(null);

  const fetchPortfolios = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<Portfolio[]>('get_portfolios', { usuario_id: userId });
      setPortfolios(result);
    } catch (e: any) {
      setError('Error al cargar portafolios');
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchPortfolios();
    // eslint-disable-next-line
  }, [userId]);

  const handleSelect = (portfolio: Portfolio) => {
    onPortfolioSelected(portfolio);
  };

  const handleAddPortfolio = async () => {
    if (!newPortfolioName.trim()) return;
    setError(null);
    console.log('[PortfolioSelectorPage] Creando portafolio', { usuario_id: userId, nombre: newPortfolioName });
    try {
      const portfolio = await invoke<Portfolio>('create_portfolio', {
        usuario_id: userId,
        nombre: newPortfolioName,
      });
      console.log('[PortfolioSelectorPage] Portafolio creado', portfolio);
      setNewPortfolioName('');
      setPortfolios([...portfolios, portfolio]);
      onPortfolioSelected(portfolio);
    } catch (e: any) {
      setError('No se pudo crear el portafolio');
      console.error('[PortfolioSelectorPage] Error al crear portafolio', e);
    }
  };

  return (
    <div className="portfolio-selector-page">
      <h2>Selecciona tu portafolio</h2>
      {loading ? (
        <p>Cargando portafolios...</p>
      ) : (
        <ul className="portfolio-list">
          {portfolios.map((p) => (
            <li key={p.id}>
              <button onClick={() => handleSelect(p)}>{p.nombre}</button>
            </li>
          ))}
        </ul>
      )}
      <div className="add-portfolio-section">
        <input
          type="text"
          placeholder="Nombre del portafolio"
          value={newPortfolioName}
          onChange={(e) => setNewPortfolioName(e.target.value)}
        />
        <button onClick={handleAddPortfolio}>Agregar portafolio</button>
      </div>
      {error && <div className="error">{error}</div>}
    </div>
  );
};

export default PortfolioSelectorPage;
