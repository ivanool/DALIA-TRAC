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
      // No se requiere userId, el backend asume MAKIMA
      const result = await invoke<Portfolio[]>('get_portfolios');
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
    try {
      // Log para depuración
      console.log('[PortfolioSelectorPage] Enviando a backend:', { nombre: newPortfolioName });
      const portfolio = await invoke<Portfolio>('create_portfolio', {
        nombre: newPortfolioName,
      });
      setNewPortfolioName('');
      setPortfolios([...portfolios, portfolio]);
      onPortfolioSelected(portfolio);
    } catch (e: any) {
      setError(e?.toString() || 'No se pudo crear el portafolio');
      console.error('[PortfolioSelectorPage] Error al crear portafolio', e);
    }
  };

  return (
    <div className="portfolio-selector-page" style={{maxWidth:600,margin:'2rem auto',padding:'2rem',background:'#fff',borderRadius:16,boxShadow:'0 2px 16px #0001'}}>
      <h2 style={{textAlign:'center',marginBottom:32}}>Selecciona tu portafolio</h2>
      {loading ? (
        <p>Cargando portafolios...</p>
      ) : portfolios.length === 0 ? (
        <p style={{textAlign:'center'}}>No tienes portafolios aún.</p>
      ) : (
        <div style={{display:'flex',flexWrap:'wrap',gap:16,justifyContent:'center',marginBottom:32}}>
          {portfolios.map((p) => (
            <div key={p.id} style={{borderRadius:32,background:'#f3f6fa',padding:'1.2rem 2.5rem',boxShadow:'0 1px 6px #0001',cursor:'pointer',transition:'0.2s',fontWeight:600,fontSize:'1.1rem'}} onClick={()=>handleSelect(p)}>
              <span role="img" aria-label="portfolio" style={{marginRight:8}}>💼</span>{p.nombre}
            </div>
          ))}
        </div>
      )}
      <div className="add-portfolio-section" style={{display:'flex',gap:8,justifyContent:'center',marginBottom:32}}>
        <input
          type="text"
          placeholder="Nombre del portafolio"
          value={newPortfolioName}
          onChange={(e) => setNewPortfolioName(e.target.value)}
          style={{borderRadius:20,padding:'0.5rem 1rem',border:'1px solid #ccc',fontSize:'1rem'}}
        />
        <button onClick={handleAddPortfolio} style={{borderRadius:20,padding:'0.5rem 1.5rem',background:'#1976d2',color:'#fff',border:'none',fontWeight:600}}>Agregar</button>
      </div>
      {portfolios.length > 0 && (
        <table style={{width:'100%',borderCollapse:'collapse',background:'#f9fafb',borderRadius:12,overflow:'hidden',boxShadow:'0 1px 6px #0001'}}>
          <thead>
            <tr style={{background:'#e3eaf2'}}>
              <th style={{padding:'0.7rem'}}>Nombre</th>
              <th style={{padding:'0.7rem'}}>ID</th>
              <th style={{padding:'0.7rem'}}>ID Hex</th>
            </tr>
          </thead>
          <tbody>
            {portfolios.map((p) => (
              <tr key={p.id} style={{textAlign:'center',cursor:'pointer'}} onClick={()=>handleSelect(p)}>
                <td style={{padding:'0.7rem',fontWeight:500}}>{p.nombre}</td>
                <td style={{padding:'0.7rem'}}>{p.id}</td>
                <td style={{padding:'0.7rem',fontFamily:'monospace'}}>{p.id_hex}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
      {error && <div className="error" style={{color:'#c00',marginTop:16,textAlign:'center'}}>{error}</div>}
    </div>
  );
};

export default PortfolioSelectorPage;
