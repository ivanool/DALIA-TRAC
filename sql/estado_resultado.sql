CREATE TABLE IF NOT EXISTS public.estado_resultado_trimestral
(
    id integer NOT NULL DEFAULT nextval('estado_resultado_acumulado_id_seq'::regclass),
    emisora text COLLATE pg_catalog."default" NOT NULL,
    trimestre text COLLATE pg_catalog."default" NOT NULL,
    fecha date,
    revenue double precision,
    grossprofit double precision,
    profitlossfromoperatingactivities double precision,
    profitloss double precision,
    profitlossbeforetax double precision,
    costofsales double precision,
    distributioncosts double precision,
    administrativeexpense double precision,
    financecosts double precision,
    financeincome double precision,
    incometaxexpensecontinuingoperations double precision,
    profitlossattributabletoownersofparent double precision,
    basicearningslosspershare double precision,
    dilutedearningslosspershare double precision,
    otherincome double precision,
    shareofprofitlossofassociatesandjointventuresaccountedforusinge double precision,
    profitlossfromdiscontinuedoperations double precision,
    depreciacion double precision,
    emisoras text COLLATE pg_catalog."default",
    serie text COLLATE pg_catalog."default",
    CONSTRAINT estado_resultado_acumulado_pkey PRIMARY KEY (id),
    CONSTRAINT estado_resultado_acumulado_emisora_trimestre_key UNIQUE (emisora, trimestre),
    CONSTRAINT estado_resultado_acumulado_emisora_serie_fkey FOREIGN KEY (emisoras, serie)
        REFERENCES public.emisoras (emisoras, serie) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.estado_resultado_trimestral
    OWNER to garden_admin;
