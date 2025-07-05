CREATE TABLE IF NOT EXISTS public.portafolios
(
    id integer NOT NULL DEFAULT nextval('portafolios_id_seq'::regclass),
    usuario_id integer,
    nombre text COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp without time zone DEFAULT now(),
    updated_at timestamp without time zone DEFAULT now(),
    id_hex text COLLATE pg_catalog."default",
    CONSTRAINT portafolios_pkey PRIMARY KEY (id),
    CONSTRAINT portafolios_id_hex_key UNIQUE (id_hex),
    CONSTRAINT portafolios_nombre_key UNIQUE (nombre),
    CONSTRAINT portafolios_usuario_id_fkey FOREIGN KEY (usuario_id)
        REFERENCES public.usuarios (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
)

TABLESPACE pg_default;

ALTER TABLE public.portafolios
    OWNER to garden_admin;
