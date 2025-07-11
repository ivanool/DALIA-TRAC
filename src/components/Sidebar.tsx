// src/components/Sidebar.tsx
import React, { useState } from 'react';
import './Sidebar.css';
import { FiPieChart, FiSearch, FiSettings, FiLogOut } from 'react-icons/fi';
import Tooltip from './Tooltip';

export type View = 'search' | 'portfolio' | 'settings';

interface SidebarProps {
  currentView: View;
  onNavigate: (view: View) => void;
}

const Sidebar: React.FC<SidebarProps> = ({ currentView, onNavigate }) => {
  const [isHovered, setIsHovered] = useState(false);
  const sidebarClass = isHovered ? 'sidebar expanded' : 'sidebar';

  // Tooltips solo cuando está colapsada
  const withTooltip = (text: string, node: React.ReactNode) =>
    isHovered ? node : <Tooltip text={text}>{node}</Tooltip>;

  return (
    <aside
      className={sidebarClass}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {/* --- Grupo Superior con el Nuevo Logo --- */}
      <div className="sidebar-top">
        <div className="sidebar-logo" onClick={() => onNavigate('portfolio')} title="Ir al Portafolio">
          {/* INICIO: Logo de Dalia estilizada */}
          <svg
            width="44"
            height="44"
            viewBox="0 0 100 100"
            xmlns="http://www.w3.org/2000/svg"
            className="sidebar-logo-svg"
          >
            <g transform="translate(50 50)">
              {/* Capa exterior de pétalos grandes */}
              {Array.from({length: 16}).map((_, i) => (
                <path
                  key={`petal-outer-${i}`}
                  d="M0,-40 Q8,-30 0,-20 Q-8,-30 0,-40 Z"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2.2"
                  transform={`rotate(${i * 22.5})`}
                />
              ))}
              {/* Capa intermedia de pétalos medianos */}
              {Array.from({length: 12}).map((_, i) => (
                <path
                  key={`petal-mid-${i}`}
                  d="M0,-28 Q5,-20 0,-12 Q-5,-20 0,-28 Z"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2.2"
                  transform={`rotate(${i * 30})`}
                />
              ))}
              {/* Capa interior de pétalos pequeños */}
              {Array.from({length: 8}).map((_, i) => (
                <path
                  key={`petal-inner-${i}`}
                  d="M0,-15 Q3,-10 0,-5 Q-3,-10 0,-15 Z"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2.2"
                  transform={`rotate(${i * 45})`}
                />
              ))}
              {/* Centro */}
              <circle cx="0" cy="0" r="5.5" fill="currentColor" opacity="0.18"/>
              <circle cx="0" cy="0" r="2.2" fill="currentColor"/>
            </g>
          </svg>
          {/* FIN: Logo de Dalia estilizada */}
        </div>
        <nav className="sidebar-nav">
          {withTooltip('Buscar',
            <button onClick={() => onNavigate('search')} className={currentView === 'search' ? 'active' : ''}>
              <FiSearch size={24} />
              <span className="nav-text">Buscar</span>
            </button>
          )}
          {withTooltip('Portafolio',
            <button onClick={() => onNavigate('portfolio')} className={currentView === 'portfolio' ? 'active' : ''}>
              <FiPieChart size={24} />
              <span className="nav-text">Portafolio</span>
            </button>
          )}
        </nav>
      </div>
      <div className="sidebar-bottom">
        <nav className="sidebar-nav">
          {withTooltip('Configuración',
            <button onClick={() => onNavigate('settings')} className={currentView === 'settings' ? 'active' : ''}>
              <FiSettings size={24} />
              <span className="nav-text">Configuración</span>
            </button>
          )}
          {withTooltip('Cerrar Sesión',
            <button className="user-profile">
              <FiLogOut size={24} />
              <span className="nav-text">Cerrar Sesión</span>
            </button>
          )}
        </nav>
      </div>
    </aside>
  );
};

export default Sidebar;
