CREATE TABLE IF NOT EXISTS public.usuarios
(
    id integer NOT NULL DEFAULT nextval('usuarios_id_seq'::regclass),
    nombre text COLLATE pg_catalog."default" NOT NULL,
    email text COLLATE pg_catalog."default",
    CONSTRAINT usuarios_pkey PRIMARY KEY (id),
    CONSTRAINT usuarios_email_key UNIQUE (email)
)

TABLESPACE pg_default;

ALTER TABLE public.usuarios
    OWNER to garden_admin;
