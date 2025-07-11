import React, { useState, useEffect, memo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './AssetViewPage.css';

// --- Interfaces para los datos (deben coincidir con el backend) ---
interface IntradiaData {
  price: number;
  open: number;
  high: number;
  low: number;
  volume: number;
  change: number;
  change_percent: number;
}

interface FinancialStatement {
  anio: number;
  trimestre: string;
  utilidad_neta: number;
  flujo_operativo: number;
  depreciacion: number;
  cambio_inventarios: number;
  impuestos_pagados: number;
}

interface AssetDetails {
  razon_social: string;
  emisoras: string;
  serie: string;
  intradia: IntradiaData;
  quarterly_financials: FinancialStatement[];
}

interface AssetViewPageProps {
  ticker: string;
  onBack: () => void;
}

// Componente del Gráfico de TradingView (memoizado para evitar re-renders)
const TradingViewChart = memo(({ ticker }: { ticker: string }) => {
  useEffect(() => {
    const script = document.createElement('script');
    script.src = 'https://s3.tradingview.com/tv.js';
    script.async = true;
    script.onload = () => {
      // @ts-ignore
      new window.TradingView.widget({
        width: '100%',
        height: 500,
        symbol: `BMV:${ticker}`,
        interval: 'D',
        timezone: 'Etc/UTC',
        theme: 'dark',
        style: '1',
        locale: 'es',
        toolbar_bg: '#f1f3f6',
        enable_publishing: false,
        allow_symbol_change: true,
        container_id: 'tradingview_chart_container',
      });
    };
    document.getElementById('tradingview_chart_container')?.appendChild(script);
  }, [ticker]);

  return <div id="tradingview_chart_container" className="tradingview-container" />;
});

const AssetViewPage: React.FC<AssetViewPageProps> = ({ ticker, onBack }) => {
  const [details, setDetails] = useState<AssetDetails | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchDetails = async () => {
      setLoading(true);
      try {
        const result: AssetDetails = await invoke('get_asset_details', { ticker });
        setDetails(result);
      } catch (err: any) {
        setError(err.toString());
      } finally {
        setLoading(false);
      }
    };
    fetchDetails();
  }, [ticker]);

  if (loading) return <div className="asset-view-page loading">Cargando datos de {ticker}...</div>;
  if (error) return <div className="asset-view-page error">Error: {error}</div>;
  if (!details) return null;

  const { intradia, quarterly_financials } = details;

  return (
    <div className="asset-view-page">
      <div className="asset-header">
        <div>
          <h1>{details.razon_social} <span className="ticker-symbol">{details.emisoras}</span></h1>
          <button onClick={onBack} className="back-button">← Volver a la búsqueda</button>
        </div>
        <div className="intradia-price">
          <h2>Último precio: {intradia.price.toFixed(2)}</h2>
          <div className="intradia-extra">
            <span>Precio promedio: <b>{intradia.open.toFixed(2)}</b></span>
            {/* Puedes agregar aquí más datos si lo deseas */}
          </div>
          <span className={intradia.change >= 0 ? 'positive' : 'negative'}>
            {intradia.change.toFixed(2)} ({intradia.change_percent.toFixed(2)}%)
          </span>
        </div>
      </div>
      <div className="key-stats-bar">
        {/* Aquí puedes agregar más estadísticas intradía si lo deseas */}
      </div>
      <TradingViewChart ticker={ticker} />
      <div className="financials-section">
        <h3>Análisis Financiero Trimestral</h3>
        <div className="financials-table-container">
          <table>
            <thead>
              <tr>
                <th>Métrica</th>
                {quarterly_financials.map(q => <th key={q.trimestre + q.anio}>{q.trimestre} {q.anio}</th>)}
              </tr>
            </thead>
            <tbody>
              {/* <tr>
                <td>Ingresos Totales</td>
                {quarterly_financials.map(q => <td key={q.trimestre + q.anio + 'ing'}>${(q.ingresos_totales / 1e6).toFixed(2)}M</td>)}
              </tr> */}
              <tr>
                <td>Utilidad Neta</td>
                {quarterly_financials.map(q => <td key={q.trimestre + q.anio + 'net'}>${(q.utilidad_neta / 1e6).toFixed(2)}M</td>)}
              </tr>
              <tr>
                <td>Flujo Operativo</td>
                {quarterly_financials.map(q => <td key={q.trimestre + q.anio + 'flujo'}>${(q.flujo_operativo / 1e6).toFixed(2)}M</td>)}
              </tr>
              <tr>
                <td>Depreciación</td>
                {quarterly_financials.map(q => <td key={q.trimestre + q.anio + 'depr'}>${(q.depreciacion / 1e6).toFixed(2)}M</td>)}
              </tr>
              <tr>
                <td>Cambio en Inventarios</td>
                {quarterly_financials.map(q => <td key={q.trimestre + q.anio + 'inv'}>${(q.cambio_inventarios / 1e6).toFixed(2)}M</td>)}
              </tr>
              <tr>
                <td>Impuestos Pagados</td>
                {quarterly_financials.map(q => <td key={q.trimestre + q.anio + 'imp'}>${(q.impuestos_pagados / 1e6).toFixed(2)}M</td>)}
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default AssetViewPage;
