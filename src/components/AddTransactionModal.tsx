import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  open: boolean;
  onClose: () => void;
  portfolio_id: number;
  userId: number;
  onTransactionAdded?: () => void;
}

const transactionTypes = [
  { value: "BUY", label: "Compra" },
  { value: "SELL", label: "Venta" },
  { value: "DIVIDEND", label: "Dividendo" },
  { value: "DEPOSIT", label: "Depósito" },
  { value: "WITHDRAWAL", label: "Retiro" },
];

const AddTransactionModal: React.FC<Props> = ({ open, onClose, portfolio_id, userId, onTransactionAdded }) => {
  const [ticker, setTicker] = useState("");
  const [type, setType] = useState("BUY");
  const [quantity, setQuantity] = useState(0);
  const [price, setPrice] = useState<number | undefined>(undefined);
  const [date, setDate] = useState("");
  const [total, setTotal] = useState(0);
  const [currency, setCurrency] = useState("MXN");
  const [notes, setNotes] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [touched, setTouched] = useState<{[key:string]: boolean}>({});

  if (!open) return null;

  const isAssetTx = type === "BUY" || type === "SELL" || type === "DIVIDEND";
  const isCashTx = type === "DEPOSIT" || type === "WITHDRAWAL";

  const validate = () => {
    if (isAssetTx && !ticker.trim()) return false;
    if (!date) return false;
    if (isAssetTx && quantity <= 0) return false;
    if (isAssetTx && (type !== "DIVIDEND") && (!price || price <= 0)) return false;
    if (total === 0) return false;
    return true;
  };

  const handleBlur = (field: string) => setTouched(t => ({ ...t, [field]: true }));

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setTouched({ ticker: true, quantity: true, price: true, date: true, total: true });
    if (!validate()) return setError("Por favor completa todos los campos obligatorios correctamente.");
    setLoading(true);
    setError(null);
    try {
      await invoke("add_portfolio_transaction", {
        portfolio_id: portfolio_id,
        user_id: userId,
        ticker,
        transaction_type: type,
        quantity,
        price,
        transaction_date: date,
        total_amount: total,
        currency,
        notes,
      });
      if (onTransactionAdded) onTransactionAdded();
      onClose();
    } catch (err) {
      setError("Error al guardar la transacción");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="modal-bg">
      <div className="modal modal-bloomberg">
        <h2 className="modal-title">Añadir Transacción</h2>
        <form className="modal-form-grid" onSubmit={handleSubmit} autoComplete="off">
          {isAssetTx && (
            <div className="form-group">
              <label>Ticker
                <input className={touched.ticker && !ticker ? "input-error" : ""} value={ticker} onChange={e => setTicker(e.target.value.toUpperCase())} onBlur={() => handleBlur('ticker')} required={isAssetTx} autoFocus />
              </label>
            </div>
          )}
          <div className="form-group">
            <label>Tipo
              <select value={type} onChange={e => setType(e.target.value)}>
                {transactionTypes.map(opt => <option key={opt.value} value={opt.value}>{opt.label}</option>)}
              </select>
            </label>
          </div>
          {isAssetTx && (
            <div className="form-group">
              <label>Cantidad
                <input type="number" className={touched.quantity && quantity <= 0 ? "input-error" : ""} value={quantity} min={0} step={type === "DIVIDEND" ? 0.01 : 1} onChange={e => setQuantity(Number(e.target.value))} onBlur={() => handleBlur('quantity')} required={isAssetTx} />
              </label>
            </div>
          )}
          {isAssetTx && type !== "DIVIDEND" && (
            <div className="form-group">
              <label>Precio
                <input type="number" className={touched.price && (!price || price <= 0) ? "input-error" : ""} value={price ?? ''} min={0} step={0.0001} onChange={e => setPrice(e.target.value ? Number(e.target.value) : undefined)} onBlur={() => handleBlur('price')} required={isAssetTx && type !== "DIVIDEND"} />
              </label>
            </div>
          )}
          <div className="form-group">
            <label>Fecha
              <input type="datetime-local" className={touched.date && !date ? "input-error" : ""} value={date} onChange={e => setDate(e.target.value)} onBlur={() => handleBlur('date')} required />
            </label>
          </div>
          <div className="form-group">
            <label>Total
              <input type="number" className={touched.total && total === 0 ? "input-error" : ""} value={total} min={0} step={0.01} onChange={e => setTotal(Number(e.target.value))} onBlur={() => handleBlur('total')} required />
            </label>
          </div>
          <div className="form-group">
            <label>Moneda
              <input value={currency} onChange={e => setCurrency(e.target.value.toUpperCase())} maxLength={3} />
            </label>
          </div>
          <div className="form-group form-notes">
            <label>Notas
              <input value={notes} onChange={e => setNotes(e.target.value)} />
            </label>
          </div>
          {error && <div className="form-error">{error}</div>}
          <div className="form-actions">
            <button type="submit" className="btn-primary" disabled={loading}>{loading ? "Guardando..." : "Guardar"}</button>
            <button type="button" className="btn-secondary" onClick={onClose}>Cancelar</button>
          </div>
        </form>
      </div>
      <style>{`
        .modal-bg {
          position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
          background: rgba(10,10,20,0.85); z-index: 1000; display: flex; align-items: center; justify-content: center;
        }
        .modal-bloomberg {
          background: #181c20; color: #e0e0e0; border-radius: 12px; box-shadow: 0 4px 32px #000a; padding: 2.5rem 2.5rem 2rem 2.5rem; min-width: 420px; max-width: 98vw;
        }
        .modal-title { margin-bottom: 1.5rem; font-size: 1.35rem; font-weight: 600; letter-spacing: 0.01em; }
        .modal-form-grid {
          display: grid; grid-template-columns: 1fr 1fr; gap: 1.1rem 1.5rem; align-items: end;
        }
        .form-group { display: flex; flex-direction: column; gap: 0.2rem; }
        .form-group label { font-size: 0.98rem; font-weight: 500; color: #b0b8c0; }
        .form-group input, .form-group select {
          background: #23272b; color: #e0e0e0; border: 1.5px solid #23272b; border-radius: 5px; padding: 0.45rem 0.7rem; font-size: 1rem; transition: border 0.2s;
        }
        .form-group input:focus, .form-group select:focus { border: 1.5px solid #ffb300; outline: none; }
        .input-error { border: 1.5px solid #ff3b3b !important; background: #2a1818; }
        .form-error { grid-column: 1 / -1; color: #ff3b3b; font-size: 1rem; margin: 0.5rem 0 0.2rem 0; text-align: center; }
        .form-actions {
          grid-column: 1 / -1; display: flex; gap: 1.2rem; justify-content: flex-end; margin-top: 1.2rem;
        }
        .btn-primary {
          background: linear-gradient(90deg, #ffb300 60%, #ff8c00 100%); color: #181c20; font-weight: 600;
          border: none; border-radius: 5px; padding: 0.6rem 1.5rem; font-size: 1.05rem; cursor: pointer; transition: background 0.2s;
        }
        .btn-primary:disabled { opacity: 0.7; cursor: not-allowed; }
        .btn-secondary {
          background: #23272b; color: #b0b8c0; border: 1.5px solid #444; border-radius: 5px; padding: 0.6rem 1.5rem; font-size: 1.05rem; cursor: pointer;
        }
        .form-notes { grid-column: 1 / -1; }
        @media (max-width: 600px) {
          .modal-bloomberg { padding: 1.2rem 0.5rem; min-width: unset; }
          .modal-form-grid { grid-template-columns: 1fr; gap: 0.8rem; }
          .form-actions { flex-direction: column; gap: 0.7rem; }
        }
      `}</style>
    </div>
  );
};

export default AddTransactionModal;
