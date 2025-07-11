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

interface FinancialDataByQuarter {
  trimestre: string;
  cashflow: Record<string, number>;
  position: Record<string, number>;
  income: Record<string, number>;
}

const EmisoraPage: React.FC<EmisoraPageProps> = ({ emisora, onBack }) => {
  const [financialData, setFinancialData] = useState<FinancialDataByQuarter[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [trimestresDisponibles, setTrimestresDisponibles] = useState<string[]>([]);

  useEffect(() => {
    // Consulta los trimestres disponibles para la emisora
    const ticker = emisora.emisoras;
    invoke<string[]>("get_trimestres_disponibles", { emisora: ticker })
      .then((trimestres) => {
        setTrimestresDisponibles(trimestres);
      })
      .catch(() => setTrimestresDisponibles([]));
  }, [emisora]);

  useEffect(() => {
    setLoading(true);
    setError(null);
    invoke("get_emisora_info", { emisora: emisora.emisoras, trimestre: null })
      .then((result: any) => {
        const data = typeof result === 'string' ? JSON.parse(result) : result;
        setFinancialData(data.datos || []);
      })
      .catch(() => {
        setError("No se pudieron cargar los datos financieros. Intenta de nuevo más tarde.");
      })
      .finally(() => setLoading(false));
  }, [emisora]);

  // Mapeo de nombres cortos para conceptos clave
  const CONCEPTOS_CLAVE: Record<string, string> = {
    // Cashflow
    flujo_operacion: "Flujo Op.",
    utilidad_neta: "Utilidad Neta",
    depreciacion: "Depreciación",
    flujo_inversion: "Flujo Inv.",
    flujo_financiamiento: "Flujo Fin.",
    efectivo_final: "Efectivo Fin.",
    // Income
    revenue: "Ingresos",
    grossprofit: "Utilidad Bruta",
    profitloss: "Utilidad Neta",
    profitlossbeforetax: "Utilidad Antes Imp.",
    basicearningslosspershare: "UPA Básica",
    // Position
    currentassets: "Activos Corr.",
    currentliabilities: "Pasivos Corr.",
    cashandcashequivalents: "Efectivo",
    equity: "Capital",
    liabilities: "Pasivos",
    propertyplantandequipment: "Prop. Planta Eq.",
    retainedearnings: "Utilidades Ret.",
  };

  const CONCEPTOS_ORDEN = [
    // Cashflow
    "flujo_operacion", "utilidad_neta", "depreciacion", "flujo_inversion", "flujo_financiamiento", "efectivo_final",
    // Income
    "revenue", "grossprofit", "profitloss", "profitlossbeforetax", "basicearningslosspershare",
    // Position
    "currentassets", "currentliabilities", "cashandcashequivalents", "equity", "liabilities", "propertyplantandequipment", "retainedearnings"
  ];

  const renderComparativeTable = (title: string, key: 'cashflow' | 'income' | 'position') => {
    if (!financialData.length) return <div>No hay datos disponibles.</div>;
    // Solo conceptos clave y en orden
    const conceptos = CONCEPTOS_ORDEN.filter(c => financialData.some(q => q[key][c] !== undefined));
    return (
      <div className="financial-section">
        <h3>{title}</h3>
        <div style={{overflowX: 'auto'}}>
          <table className="estado-table tradingview-style-table">
            <thead>
              <tr>
                <th style={{position:'sticky', left:0, background:'var(--color-background-secondary)', zIndex:2}}>Concepto</th>
                {financialData.map(q => (
                  <th key={q.trimestre} style={{minWidth:100, textAlign:'center'}}>{q.trimestre.replace(/([1-4])T_(\d{4})/, '$1T $2')}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {conceptos.map(concepto => (
                <tr key={concepto}>
                  <td style={{position:'sticky', left:0, background:'var(--color-background-secondary)', color:'var(--color-text-secondary)', fontWeight:500}}>
                    {CONCEPTOS_CLAVE[concepto] || concepto.replace(/_/g, ' ')}
                  </td>
                  {financialData.map((q, idx) => {
                    const val = q[key][concepto];
                    let color = 'var(--color-text)';
                    if (idx > 0 && val !== undefined) {
                      const prev = financialData[idx-1][key][concepto];
                      if (prev !== undefined) {
                        if (val > prev) color = 'var(--color-positive)';
                        else if (val < prev) color = 'var(--color-negative)';
                      }
                    }
                    return (
                      <td key={q.trimestre + concepto} style={{color, textAlign:'right', fontVariantNumeric:'tabular-nums', fontWeight:600, background: idx%2===0 ? 'rgba(28,33,39,0.95)' : 'rgba(16,20,25,0.95)'}}>
                        {val !== undefined ? val.toLocaleString('es-MX', { style: 'currency', currency: 'MXN', maximumFractionDigits: 0 }) : '-'}
                      </td>
                    );
                  })}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  };

  if (loading) return <div className="emisora-page"> <button onClick={onBack}>← Volver</button> Cargando datos financieros... </div>;
  if (error) return <div className="emisora-page"> <button onClick={onBack}>← Volver</button> <p>{error}</p></div>;

  return (
    <div className="emisora-page">
      <button onClick={onBack}>← Volver</button>
      <h2>{emisora.razon_social} <span style={{color:'#888'}}>{emisora.emisoras}{emisora.serie ? '.'+emisora.serie : ''}</span></h2>
      {trimestresDisponibles.length === 0 ? (
        <div style={{marginTop: '2rem', color: '#b00', fontWeight: 'bold'}}>
          No hay trimestres disponibles para esta emisora.
        </div>
      ) : (
        <div className="estados-financieros">
          {renderComparativeTable("Flujos Financieros", 'cashflow')}
          {renderComparativeTable("Resultados Trimestrales", 'income')}
          {renderComparativeTable("Posición Financiera", 'position')}
        </div>
      )}
    </div>
  );
};

export default EmisoraPage;
