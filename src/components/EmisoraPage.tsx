import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./EmisoraPage.css";

interface EmisoraBusqueda {
  razon_social: string;
  emisoras: string;
  serie: string;
}

interface EmisoraPageProps {
  emisora: EmisoraBusqueda;
  onBack: () => void;
}

const EmisoraPage: React.FC<EmisoraPageProps> = ({ emisora, onBack }) => {
  const [info, setInfo] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [flujo, setFlujo] = useState<any[]>([]);
  const [posicion, setPosicion] = useState<any[]>([]);
  const [resultado, setResultado] = useState<any[]>([]);
  const [finLoading, setFinLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    // Enviar el string concatenado para robustez
    const tickerSerie = `${emisora.emisoras}${emisora.serie || ''}`;
    invoke('get_emisora_info', { emisora_serie: tickerSerie })
      .then((data: any) => {
        setInfo(data);
        setError(null);
      })
      .catch(() => setError('No se pudo cargar la información de la emisora'))
      .finally(() => setLoading(false));
    setFinLoading(true);
    Promise.all([
      invoke('get_estado_flujo', { emisora_serie: tickerSerie }),
      invoke('get_estado_posicion', { emisora_serie: tickerSerie }),
      invoke('get_estado_resultado', { emisora_serie: tickerSerie })
    ]).then(([flujoData, posData, resData]) => {
      setFlujo(Array.isArray(flujoData) ? flujoData : []);
      setPosicion(Array.isArray(posData) ? posData : []);
      setResultado(Array.isArray(resData) ? resData : []);
    }).finally(() => setFinLoading(false));
  }, [emisora]);

  if (loading) return <div className="emisora-page"><button onClick={onBack}>← Volver</button> Cargando...</div>;
  if (error) return <div className="emisora-page"><button onClick={onBack}>← Volver</button> {error}</div>;
  if (!info) return null;

  return (
    <div className="emisora-page">
      <button onClick={onBack}>← Volver</button>
      <h2>{info.razon_social} <span style={{color:'#888'}}>{info.emisoras}{info.serie ? '.'+info.serie : ''}</span></h2>
      <ul className="emisora-info-list">
        <li><b>ISIN:</b> {info.isin}</li>
        <li><b>Bolsa:</b> {info.bolsa}</li>
        <li><b>Tipo valor:</b> {info.tipo_valor_descripcion || info.tipo_valor}</li>
        <li><b>Estatus:</b> {info.estatus}</li>
        <li><b>Acciones en circulación:</b> {info.acciones_circulacion ?? 'N/A'}</li>
      </ul>
      <div className="estados-financieros">
        <h3>Estado de Flujo</h3>
        {finLoading ? <div>Cargando...</div> : (
          flujo.length ? (
            <table className="estado-table"><thead><tr>{Object.keys(flujo[0]||{}).map(k => <th key={k}>{k}</th>)}</tr></thead><tbody>{flujo.map((row,i) => <tr key={i}>{Object.values(row).map((v,j) => <td key={j}>{String(v ?? '')}</td>)}</tr>)}</tbody></table>
          ) : <div>No hay datos de flujo.</div>
        )}
        <h3>Estado de Posición</h3>
        {finLoading ? <div>Cargando...</div> : (
          posicion.length ? (
            <table className="estado-table"><thead><tr>{Object.keys(posicion[0]||{}).map(k => <th key={k}>{k}</th>)}</tr></thead><tbody>{posicion.map((row,i) => <tr key={i}>{Object.values(row).map((v,j) => <td key={j}>{String(v ?? '')}</td>)}</tr>)}</tbody></table>
          ) : <div>No hay datos de posición.</div>
        )}
        <h3>Estado de Resultado</h3>
        {finLoading ? <div>Cargando...</div> : (
          resultado.length ? (
            <table className="estado-table"><thead><tr>{Object.keys(resultado[0]||{}).map(k => <th key={k}>{k}</th>)}</tr></thead><tbody>{resultado.map((row,i) => <tr key={i}>{Object.values(row).map((v,j) => <td key={j}>{String(v ?? '')}</td>)}</tr>)}</tbody></table>
          ) : <div>No hay datos de resultado.</div>
        )}
      </div>
    </div>
  );
};

export default EmisoraPage;
