import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './SearchBar.css';

interface EmisoraBusqueda {
  razon_social: string;
  emisoras: string;
  serie: string;
}

interface SearchBarProps {
  onSelect: (emisora: EmisoraBusqueda) => void;
}

const SearchBar: React.FC<SearchBarProps> = ({ onSelect }) => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<EmisoraBusqueda[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSearch = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setQuery(value);
    if (value.length < 2) {
      setResults([]);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const res: EmisoraBusqueda[] = await invoke('get_emisora_query', { query: value });
      setResults(res);
    } catch (err: any) {
      setError('Error al buscar');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="search-bar-container">
      <input
        type="text"
        placeholder="Buscar activo o emisora..."
        value={query}
        onChange={handleSearch}
        className="search-bar-input"
      />
      {loading && <div className="search-bar-loading">Buscando...</div>}
      {error && <div className="search-bar-error">{error}</div>}
      {results.length > 0 && (
        <ul className="search-bar-results">
          {results.map((item) => (
            <li key={item.emisoras + item.serie}
                onClick={() => onSelect(item)}
                className="search-bar-result-item">
              <span className="search-bar-symbol">{item.emisoras}</span>
              <span className="search-bar-name">{item.razon_social}</span>
              <span className="search-bar-serie">{item.serie}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default SearchBar;
