CREATE TABLE IF NOT EXISTS public.dividendos
(
    id integer NOT NULL DEFAULT nextval('dividendos_id_seq'::regclass),
    portafolio_ticker_id integer,
    monto double precision NOT NULL,
    fecha timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT dividendos_pkey PRIMARY KEY (id),
    CONSTRAINT dividendos_portafolio_ticker_id_fkey FOREIGN KEY (portafolio_ticker_id)
        REFERENCES public.portafolio_ticker (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.dividendos
    OWNER to garden_admin;
