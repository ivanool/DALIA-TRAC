CREATE TABLE IF NOT EXISTS public.emisoras
(
    emisoras text COLLATE pg_catalog."default",
    serie text COLLATE pg_catalog."default",
    razon_social text COLLATE pg_catalog."default",
    isin text COLLATE pg_catalog."default",
    bolsa text COLLATE pg_catalog."default",
    tipo_valor text COLLATE pg_catalog."default",
    tipo_valor_id text COLLATE pg_catalog."default",
    estatus text COLLATE pg_catalog."default",
    acciones_circulacion bigint,
    rangos_historicos text COLLATE pg_catalog."default",
    rangos_financieros text COLLATE pg_catalog."default",
    dividendos text COLLATE pg_catalog."default",
    CONSTRAINT emisoras_serie_unique UNIQUE (emisoras, serie)
)

TABLESPACE pg_default;

ALTER TABLE public.emisoras
    OWNER to garden_admin;
