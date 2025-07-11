import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './TickerTape.css';

interface TickerItem {
  symbol: string;
  price: number;
  change: number;
  change_percent: number;
}

interface TickerTapeProps {
  onTickerClick?: (symbol: string) => void;
}

const TickerTape: React.FC<TickerTapeProps> = ({ onTickerClick }) => {
  const [items, setItems] = useState<TickerItem[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchTickerData = async () => {
      try {
        const result: TickerItem[] = await invoke('get_ticker_data');
        setItems(result);
      } catch (error) {
        setItems([
          { symbol: 'IPC', price: 52874.64, change: 150.4, change_percent: 0.28 },
          { symbol: 'S&P 500', price: 5487.03, change: -1.5, change_percent: -0.27 },
          { symbol: 'AMXL', price: 16.5, change: 0.25, change_percent: 1.54 },
          { symbol: 'WALMEX.MX', price: 68.9, change: -0.1, change_percent: -0.15 },
        ]);
      } finally {
        setLoading(false);
      }
    };
    fetchTickerData();
  }, []);

  const renderItem = (item: TickerItem, isClone = false) => {
    const isPositive = item.change_percent >= 0;
    const changeClass = isPositive ? 'positive' : 'negative';
    const arrow = isPositive ? '▲' : '▼';
    const key = isClone ? `${item.symbol}-clone` : item.symbol;
    return (
      <div
        className="ticker__item"
        key={key}
        onClick={() => onTickerClick && onTickerClick(item.symbol)}
        style={{ cursor: onTickerClick ? 'pointer' : 'default' }}
      >
        <span className="ticker__symbol">{item.symbol}</span>
        <span className="ticker__price">{item.price.toFixed(2)}</span>
        <span className={`ticker__change ${changeClass}`}>{item.change_percent.toFixed(2)}% {arrow}</span>
      </div>
    );
  };

  if (loading) {
    return <div className="ticker-wrap loading">Cargando...</div>;
  }

  return (
    <div className="ticker-wrap">
      <div className="ticker">
        {/* Renderizamos la lista dos veces para la animación de bucle infinito */}
        {items.map(item => renderItem(item, false))}
        {items.map(item => renderItem(item, true))}
      </div>
    </div>
  );
};

export default TickerTape;
