CREATE TABLE IF NOT EXISTS public.estado_posicion
(
    emisora text COLLATE pg_catalog."default" NOT NULL,
    trimestre text COLLATE pg_catalog."default" NOT NULL,
    fecha date,
    currentassets double precision,
    currentliabilities double precision,
    cashandcashequivalents double precision,
    inventories double precision,
    tradeandothercurrentreceivables double precision,
    tradeandothercurrentpayables double precision,
    equity double precision,
    liabilities double precision,
    noncurrentliabilities double precision,
    equityattributabletoownersofparent double precision,
    noncontrollinginterests double precision,
    propertyplantandequipment double precision,
    intangibleassetsotherthangoodwill double precision,
    goodwill double precision,
    rightofuseassetsthatdonotmeetdefinitionofinvestmentproperty double precision,
    deferredtaxassets double precision,
    deferredtaxliabilities double precision,
    noncurrentassetsordisposalgroupsclassifiedasheldforsale double precision,
    retainedearnings double precision,
    issuedcapital double precision,
    otherreserves double precision,
    noncurrentleaseliabilities double precision,
    othernoncurrentfinancialliabilities double precision,
    noncurrentprovisionsforemployeebenefits double precision,
    emisoras text COLLATE pg_catalog."default",
    serie text COLLATE pg_catalog."default",
    CONSTRAINT estado_posicion_pkey PRIMARY KEY (emisora, trimestre),
    CONSTRAINT estado_posicion_emisora_serie_fkey FOREIGN KEY (emisoras, serie)
        REFERENCES public.emisoras (emisoras, serie) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.estado_posicion
    OWNER to garden_admin;
