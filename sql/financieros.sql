
-- Tabla: estado_flujos (Flujos de Efectivo)
DROP TABLE IF EXISTS estado_flujos CASCADE;
CREATE TABLE estado_flujos (
    emisora TEXT NOT NULL REFERENCES emisoras(emisoras),
    trimestre TEXT NOT NULL,
    fecha DATE,
    flujo_operacion DECIMAL,
    utilidad_neta DECIMAL,
    depreciacion DECIMAL,
    cambio_inventarios DECIMAL,
    cambio_cxc DECIMAL,
    cambio_cxp DECIMAL,
    impuestos_pagados DECIMAL,
    intereses_pagados DECIMAL,
    flujo_inversion DECIMAL,
    capex DECIMAL,
    venta_activos DECIMAL,
    compra_intangibles DECIMAL,
    flujo_financiamiento DECIMAL,
    prestamos_obtenidos DECIMAL,
    pago_deuda DECIMAL,
    dividendos_pagados DECIMAL,
    recompras DECIMAL,
    cambio_efectivo DECIMAL,
    efectivo_final DECIMAL,
    efecto_tc DECIMAL,
    deterioros DECIMAL,
    partidas_no_monetarias DECIMAL,
    costos_financieros DECIMAL,
    datos JSONB,
    PRIMARY KEY (emisora, trimestre)
);
