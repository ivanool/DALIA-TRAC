CREATE TABLE IF NOT EXISTS public.portafolio_ticker
(
    id integer NOT NULL DEFAULT nextval('portafolio_ticker_id_seq'::regclass),
    portafolio_id integer,
    ticker text COLLATE pg_catalog."default" NOT NULL,
    emisoras text COLLATE pg_catalog."default",
    serie text COLLATE pg_catalog."default",
    created_at timestamp with time zone NOT NULL DEFAULT now(), -- <--- NUEVA COLUMNA
    CONSTRAINT portafolio_ticker_pkey PRIMARY KEY (id),
    CONSTRAINT portafolio_ticker_emisora_serie_fkey FOREIGN KEY (emisoras, serie)
        REFERENCES public.emisoras (emisoras, serie) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT portafolio_ticker_portafolio_id_fkey FOREIGN KEY (portafolio_id)
        REFERENCES public.portafolios (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.portafolio_ticker
    OWNER to garden_admin;