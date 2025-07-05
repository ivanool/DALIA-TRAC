CREATE TABLE IF NOT EXISTS public.estado_flujos
(
    emisora text COLLATE pg_catalog."default" NOT NULL,
    trimestre text COLLATE pg_catalog."default" NOT NULL,
    fecha date,
    flujo_operacion double precision,
    utilidad_neta double precision,
    depreciacion double precision,
    cambio_inventarios double precision,
    cambio_cxc double precision,
    cambio_cxp double precision,
    impuestos_pagados double precision,
    intereses_pagados double precision,
    flujo_inversion double precision,
    capex double precision,
    venta_activos double precision,
    compra_intangibles double precision,
    flujo_financiamiento double precision,
    prestamos_obtenidos double precision,
    pago_deuda double precision,
    dividendos_pagados double precision,
    recompras double precision,
    cambio_efectivo double precision,
    efectivo_final double precision,
    efecto_tc double precision,
    deterioros double precision,
    partidas_no_monetarias double precision,
    costos_financieros double precision,
    emisoras text COLLATE pg_catalog."default",
    serie text COLLATE pg_catalog."default",
    CONSTRAINT estado_flujos_pkey PRIMARY KEY (emisora, trimestre),
    CONSTRAINT estado_flujos_emisora_serie_fkey FOREIGN KEY (emisoras, serie)
        REFERENCES public.emisoras (emisoras, serie) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.estado_flujos
    OWNER to garden_admin;
