CREATE TABLE IF NOT EXISTS public.cashflow
(
    id integer NOT NULL DEFAULT nextval('cashflow_id_seq'::regclass),
    portafolio_id integer,
    monto double precision NOT NULL,
    tipo text COLLATE pg_catalog."default" NOT NULL,
    fecha timestamp without time zone NOT NULL DEFAULT now(),
    descripcion text COLLATE pg_catalog."default",
    CONSTRAINT cashflow_pkey PRIMARY KEY (id),
    CONSTRAINT cashflow_portafolio_id_fkey FOREIGN KEY (portafolio_id)
        REFERENCES public.portafolios (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.cashflow
    OWNER to garden_admin;
