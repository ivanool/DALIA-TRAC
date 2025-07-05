CREATE TABLE IF NOT EXISTS public.transacciones
(
    id integer NOT NULL DEFAULT nextval('transacciones_id_seq'::regclass),
    portafolio_ticker_id integer,
    tipo text COLLATE pg_catalog."default" NOT NULL,
    cantidad integer NOT NULL,
    precio double precision NOT NULL,
    fecha timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT transacciones_pkey PRIMARY KEY (id),
    CONSTRAINT transacciones_portafolio_ticker_id_fkey FOREIGN KEY (portafolio_ticker_id)
        REFERENCES public.portafolio_ticker (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.transacciones
    OWNER to garden_admin;
