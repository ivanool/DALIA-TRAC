import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Sidebar from './components/Sidebar';
import PortfolioPage from './components/PortfolioPage';
import EmisoraPage from './components/EmisoraPage';
import TickerTape from './components/TickerTape';
import AssetViewPage from './components/AssetViewPage';
import SearchBar from './components/SearchBar';

type View = 'search' | 'portfolio' | 'settings';

function App() {
  const [currentView, setCurrentView] = useState<View>('portfolio');
  const [userId] = useState(1); // Simulación de usuario
  const [portfolioId] = useState(1); // Simulación de portafolio
  const [selectedEmisora, setSelectedEmisora] = useState<any>(null);
  const [showTransactionModal, setShowTransactionModal] = useState(false);
  const [isSidebarExpanded, setIsSidebarExpanded] = useState(false);

  const handleNavigation = (view: View) => {
    setCurrentView(view);
    setSelectedEmisora(null);
  };

  const handleTickerClick = (symbol: string) => {
    setCurrentView('search');
    setSelectedEmisora(symbol);
  };

  const handleOpenTransactionModal = () => setShowTransactionModal(true);
  const handleCloseTransactionModal = () => setShowTransactionModal(false);
  const handleToggleSidebar = () => setIsSidebarExpanded(prev => !prev);

  const renderMainContent = () => {
    switch (currentView) {
      case 'portfolio':
        return <PortfolioPage portfolioId={portfolioId} userId={userId} />;
      case 'settings':
        return <div style={{padding:'2rem'}}><h1>Configuración</h1><p>Próximamente...</p></div>;
      case 'search':
      default:
        if (selectedEmisora) {
          return <AssetViewPage ticker={selectedEmisora.emisoras} onBack={() => setSelectedEmisora(null)} />;
        }
        return (
          <div className="search-container" style={{padding:'2rem'}}>
            <h2>Búsqueda</h2>
            <SearchBar onSelect={item => setSelectedEmisora(item)} />
          </div>
        );
    }
  };

  return (
    <div className="app-layout">
      <Sidebar 
        currentView={currentView} 
        isExpanded={isSidebarExpanded}
        onNavigate={handleNavigation} 
        onOpenTransactionModal={handleOpenTransactionModal}
        onToggle={handleToggleSidebar}
      />
      <div className="main-content">
        <TickerTape onTickerClick={handleTickerClick} />
        <main>
          <div className="main-content-inner">
            {renderMainContent()}
          </div>
        </main>
      </div>
      {/* Modal de transacción (puedes conectar tu modal real aquí) */}
      {showTransactionModal && (
        <div className="modal-bg" onClick={handleCloseTransactionModal}>
          <div className="modal-content" onClick={e => e.stopPropagation()}>
            <h2>Nueva Transacción</h2>
            <button onClick={handleCloseTransactionModal}>Cerrar</button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
