import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { format } from 'date-fns';

interface PositionSlot {
  ticker: string;
  total_quantity: number;
  average_price: number;
}

interface ProfitLoss {
  ticker: string;
  total_quantity: number;
  average_price: number;
  current_price: number;
  unrealized_pl: number;
  unrealized_pl_percent: number;
}

interface CashFlow {
  id: number;
  portfolio_id: number;
  flow_type: string;
  amount: number;
  flow_date: string;
  description?: string;
}

const PortfolioPage: React.FC<{ portfolio_id: number }> = ({ portfolio_id }) => {
  const [cashBalance, setCashBalance] = useState<number>(0);
  const [cashHistory, setCashHistory] = useState<CashFlow[]>([]);
  const [positions, setPositions] = useState<PositionSlot[]>([]);
  const [pl, setPL] = useState<ProfitLoss[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCashModal, setShowCashModal] = useState(false);
  const [showAssetModal, setShowAssetModal] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Fetch all portfolio data
  const fetchAll = async () => {
    setLoading(true);
    setError(null);
    try {
      const [balance, history, slots, plData] = await Promise.all([
        invoke<number>('get_cash_balance', { portfolio_id }),
        invoke<CashFlow[]>('get_cash_flow_history', { portfolio_id }),
        invoke<PositionSlot[]>('get_portfolio_slots', { portfolio_id }),
        invoke<ProfitLoss[]>('calculate_portfolio_pl', { portfolio_id })
      ]);
      setCashBalance(balance);
      setCashHistory(history);
      setPositions(slots);
      setPL(plData);
    } catch (e: any) {
      setError(e?.toString() || 'Error al cargar datos del portafolio');
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchAll();
    // eslint-disable-next-line
  }, [portfolio_id]);

  // --- UI para agregar movimiento de efectivo ---
  const [cashType, setCashType] = useState<'deposit'|'withdrawal'>('deposit');
  const [cashAmount, setCashAmount] = useState('');
  const [cashDate, setCashDate] = useState(() => format(new Date(), 'yyyy-MM-dd'));
  const [cashDesc, setCashDesc] = useState('');

  const handleAddCash = async () => {
    try {
      if (typeof portfolio_id !== 'number' || isNaN(portfolio_id)) {
        setError('ID de portafolio inválido');
        return;
      }
      await invoke('add_cash_movement', {
        portfolio_id: Number(portfolio_id),
        flow_type: cashType,
        amount: parseFloat(cashAmount),
        flow_date: cashDate,
        description: cashDesc
      });
      setShowCashModal(false);
      setCashAmount('');
      setCashDesc('');
      fetchAll();
    } catch (e: any) {
      setError(e?.toString() || 'No se pudo agregar el movimiento de efectivo');
    }
  };

  // --- UI para agregar transacción de activo ---
  const [assetTicker, setAssetTicker] = useState('');
  const [assetType, setAssetType] = useState<'buy'|'sell'>('buy');
  const [assetQty, setAssetQty] = useState('');
  const [assetPrice, setAssetPrice] = useState('');
  const [assetDate, setAssetDate] = useState(() => format(new Date(), 'yyyy-MM-dd'));
  const [assetUseCash, setAssetUseCash] = useState(true);

  const handleAddAsset = async () => {
    try {
      await invoke('add_asset_transaction', {
        portfolio_id,
        ticker: assetTicker,
        transaction_type: assetType,
        quantity: parseFloat(assetQty),
        price: parseFloat(assetPrice),
        transaction_date: assetDate,
        use_cash_from_portfolio: assetUseCash
      });
      setShowAssetModal(false);
      setAssetTicker('');
      setAssetQty('');
      setAssetPrice('');
      fetchAll();
    } catch (e: any) {
      setError(e?.toString() || 'No se pudo agregar la transacción');
    }
  };

  return (
    <div className="portfolio-page" style={{maxWidth:900,margin:'2rem auto',padding:'2rem',background:'#fff',borderRadius:16,boxShadow:'0 2px 16px #0001'}}>
      <h1 style={{textAlign:'center'}}>Mi Portafolio</h1>
      <div style={{display:'flex',gap:'2rem',marginBottom:32}}>
        <div style={{flex:1}}>
          <div style={{marginBottom:16}}>
            <b>Efectivo:</b> ${cashBalance.toLocaleString('es-MX')}
            <button style={{marginLeft:16}} onClick={()=>setShowCashModal(true)}>+ Movimiento de Efectivo</button>
          </div>
          <div style={{marginBottom:16}}>
            <b>Posiciones:</b> <button style={{marginLeft:16}} onClick={()=>setShowAssetModal(true)}>+ Transacción de Activo</button>
          </div>
          <table style={{width:'100%',marginBottom:24}}>
            <thead>
              <tr style={{background:'#e3eaf2'}}>
                <th>Ticker</th>
                <th>Cantidad</th>
                <th>Precio Promedio</th>
                <th>Precio Actual</th>
                <th>G/P No Realizado</th>
                <th>%</th>
              </tr>
            </thead>
            <tbody>
              {pl.map(slot => (
                <tr key={slot.ticker}>
                  <td>{slot.ticker}</td>
                  <td>{slot.total_quantity}</td>
                  <td>${slot.average_price.toFixed(2)}</td>
                  <td>${slot.current_price.toFixed(2)}</td>
                  <td style={{color: slot.unrealized_pl > 0 ? 'green' : 'red'}}>${slot.unrealized_pl.toFixed(2)}</td>
                  <td style={{color: slot.unrealized_pl > 0 ? 'green' : 'red'}}>{slot.unrealized_pl_percent.toFixed(2)}%</td>
                </tr>
              ))}
            </tbody>
          </table>
          <h3>Historial de Movimientos de Efectivo</h3>
          <table style={{width:'100%',marginBottom:24}}>
            <thead>
              <tr style={{background:'#e3eaf2'}}>
                <th>Fecha</th>
                <th>Tipo</th>
                <th>Monto</th>
                <th>Descripción</th>
              </tr>
            </thead>
            <tbody>
              {cashHistory.map(c => (
                <tr key={c.id}>
                  <td>{c.flow_date}</td>
                  <td>{c.flow_type}</td>
                  <td style={{color: c.amount > 0 ? 'green' : 'red'}}>${c.amount.toFixed(2)}</td>
                  <td>{c.description}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
      {/* Modal para movimiento de efectivo */}
      {showCashModal && (
        <div className="modal-bg" onClick={()=>setShowCashModal(false)}>
          <div className="modal-content" onClick={e=>e.stopPropagation()} style={{maxWidth:400}}>
            <h2>Agregar Movimiento de Efectivo</h2>
            <div style={{marginBottom:8}}>
              <select value={cashType} onChange={e=>setCashType(e.target.value as any)}>
                <option value="deposit">Depósito</option>
                <option value="withdrawal">Retiro</option>
              </select>
            </div>
            <div style={{marginBottom:8}}>
              <input type="number" placeholder="Monto" value={cashAmount} onChange={e=>setCashAmount(e.target.value)} />
            </div>
            <div style={{marginBottom:8}}>
              <input type="date" value={cashDate} onChange={e=>setCashDate(e.target.value)} />
            </div>
            <div style={{marginBottom:8}}>
              <input type="text" placeholder="Descripción" value={cashDesc} onChange={e=>setCashDesc(e.target.value)} />
            </div>
            <button onClick={handleAddCash}>Agregar</button>
            <button onClick={()=>setShowCashModal(false)} style={{marginLeft:8}}>Cancelar</button>
          </div>
        </div>
      )}
      {/* Modal para transacción de activo */}
      {showAssetModal && (
        <div className="modal-bg" onClick={()=>setShowAssetModal(false)}>
          <div className="modal-content" onClick={e=>e.stopPropagation()} style={{maxWidth:400}}>
            <h2>Agregar Transacción de Activo</h2>
            <div style={{marginBottom:8}}>
              <select value={assetType} onChange={e=>setAssetType(e.target.value as any)}>
                <option value="buy">Compra</option>
                <option value="sell">Venta</option>
              </select>
            </div>
            <div style={{marginBottom:8}}>
              <input type="text" placeholder="Ticker" value={assetTicker} onChange={e=>setAssetTicker(e.target.value)} />
            </div>
            <div style={{marginBottom:8}}>
              <input type="number" placeholder="Cantidad" value={assetQty} onChange={e=>setAssetQty(e.target.value)} />
            </div>
            <div style={{marginBottom:8}}>
              <input type="number" placeholder="Precio" value={assetPrice} onChange={e=>setAssetPrice(e.target.value)} />
            </div>
            <div style={{marginBottom:8}}>
              <input type="date" value={assetDate} onChange={e=>setAssetDate(e.target.value)} />
            </div>
            <div style={{marginBottom:8}}>
              <label><input type="checkbox" checked={assetUseCash} onChange={e=>setAssetUseCash(e.target.checked)} /> Usar efectivo del portafolio</label>
            </div>
            <button onClick={handleAddAsset}>Agregar</button>
            <button onClick={()=>setShowAssetModal(false)} style={{marginLeft:8}}>Cancelar</button>
          </div>
        </div>
      )}
      {error && <div className="error" style={{color:'#c00',marginTop:16,textAlign:'center'}}>{error}</div>}
      {loading && <div style={{textAlign:'center',marginTop:16}}>Cargando...</div>}
    </div>
  );
};

export default PortfolioPage;
