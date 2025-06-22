-- Elimina la tabla estado_flujos si existe
DROP TABLE IF EXISTS estado_flujos CASCADE;

-- Crea la tabla con los tipos correctos (FLOAT8 para todos los campos numéricos)
CREATE TABLE estado_flujos (
    emisora                TEXT    NOT NULL,
    trimestre              TEXT    NOT NULL,
    fecha                  DATE,
    flujo_operacion        FLOAT8,
    utilidad_neta          FLOAT8,
    depreciacion           FLOAT8,
    cambio_inventarios     FLOAT8,
    cambio_cxc             FLOAT8,
    cambio_cxp             FLOAT8,
    impuestos_pagados      FLOAT8,
    intereses_pagados      FLOAT8,
    flujo_inversion        FLOAT8,
    capex                  FLOAT8,
    venta_activos          FLOAT8,
    compra_intangibles     FLOAT8,
    flujo_financiamiento   FLOAT8,
    prestamos_obtenidos    FLOAT8,
    pago_deuda             FLOAT8,
    dividendos_pagados     FLOAT8,
    recompras              FLOAT8,
    cambio_efectivo        FLOAT8,
    efectivo_final         FLOAT8,
    efecto_tc              FLOAT8,
    deterioros             FLOAT8,
    partidas_no_monetarias FLOAT8,
    costos_financieros     FLOAT8,
    PRIMARY KEY (emisora, trimestre),
    FOREIGN KEY (emisora) REFERENCES emisoras(emisoras)
);
