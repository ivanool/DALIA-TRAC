import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Sidebar from './components/Sidebar';
import PortfolioPage from './components/PortfolioPage';
import EmisoraPage from './components/EmisoraPage';
import TickerTape from './components/TickerTape';
import AssetViewPage from './components/AssetViewPage';
import SearchBar from './components/SearchBar';
import UserSelectorPage from './components/UserSelectorPage';
import PortfolioSelectorPage from './components/PortfolioSelectorPage';

type View = 'search' | 'portfolio' | 'settings';

function App() {
  const [currentView, setCurrentView] = useState<View>('portfolio');
  const [selectedUser, setSelectedUser] = useState<any>(null);
  const [portfolio_id, setPortfolioId] = useState<number | null>(null);
  const [selectedEmisora, setSelectedEmisora] = useState<any>(null);
  const [showTransactionModal, setShowTransactionModal] = useState(false);
  const [isSidebarExpanded, setIsSidebarExpanded] = useState(false);
  const [showWelcome, setShowWelcome] = useState(true);

  // Pantalla de bienvenida
  if (showWelcome) {
    return (
      <div className="welcome-screen" style={{display:'flex',flexDirection:'column',alignItems:'center',justifyContent:'center',height:'100vh'}}>
        <img src="/vite.svg" alt="Logo" style={{width:80,marginBottom:24}} />
        <h1>Bienvenido a DaliaTrac</h1>
        <p>Gestión de portafolios financieros de manera sencilla y profesional.</p>
        <button style={{marginTop:32}} onClick={()=>setShowWelcome(false)}>Comenzar</button>
      </div>
    );
  }

  // Selección de usuario
  if (!selectedUser) {
    return <UserSelectorPage onUserSelected={setSelectedUser} />;
  }

  // Selección de portafolio
  if (!portfolio_id) {
    return (
      <div>
        <div style={{display:'flex',alignItems:'center',justifyContent:'space-between',padding:'1rem'}}>
          <span>Usuario: <b>{selectedUser.nombre}</b> <button onClick={()=>{setSelectedUser(null);setPortfolioId(null);}}>Cerrar sesión</button></span>
        </div>
        <PortfolioSelectorPage userId={selectedUser.id} onPortfolioSelected={p => setPortfolioId(p.id)} />
      </div>
    );
  }

  // Si portfolio_id es null o undefined, no renderizar PortfolioPage
  if (portfolio_id == null) return null;

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
        return <PortfolioPage portfolio_id={portfolio_id} />;
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
