CREATE TABLE IF NOT EXISTS public.intradia_data
(
    id integer NOT NULL DEFAULT nextval('intradia_data_id_seq'::regclass),
    emisora character varying(20) COLLATE pg_catalog."default" NOT NULL,
    fecha_hora timestamp without time zone NOT NULL,
    precio double precision NOT NULL,
    emisoras text COLLATE pg_catalog."default",
    serie text COLLATE pg_catalog."default",
    CONSTRAINT intradia_data_pkey PRIMARY KEY (id),
    CONSTRAINT intradia_data_emisora_fecha_hora_key UNIQUE (emisora, fecha_hora),
    CONSTRAINT intradia_data_emisora_serie_fkey FOREIGN KEY (emisoras, serie)
        REFERENCES public.emisoras (emisoras, serie) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.intradia_data
    OWNER to garden_admin;

-- Index: public.idx_intradia_emisora_fecha
CREATE INDEX IF NOT EXISTS idx_intradia_emisora_fecha
    ON public.intradia_data USING btree
    (emisora COLLATE pg_catalog."default" ASC NULLS LAST, fecha_hora ASC NULLS LAST)
    TABLESPACE pg_default;