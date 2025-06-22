-- Asegura que las columnas emisoras y serie existan en todas las tablas relacionadas
ALTER TABLE portafolio_ticker ADD COLUMN IF NOT EXISTS emisoras text;
ALTER TABLE portafolio_ticker ADD COLUMN IF NOT EXISTS serie text;

ALTER TABLE intradia_data ADD COLUMN IF NOT EXISTS emisoras text;
ALTER TABLE intradia_data ADD COLUMN IF NOT EXISTS serie text;

ALTER TABLE estado_posicion ADD COLUMN IF NOT EXISTS emisoras text;
ALTER TABLE estado_posicion ADD COLUMN IF NOT EXISTS serie text;

ALTER TABLE estado_resultado_trimestre ADD COLUMN IF NOT EXISTS emisoras text;
ALTER TABLE estado_resultado_trimestre ADD COLUMN IF NOT EXISTS serie text;

ALTER TABLE estado_flujos ADD COLUMN IF NOT EXISTS emisoras text;
ALTER TABLE estado_flujos ADD COLUMN IF NOT EXISTS serie text;

ALTER TABLE estado_resultado_acumulado ADD COLUMN IF NOT EXISTS emisoras text;
ALTER TABLE estado_resultado_acumulado ADD COLUMN IF NOT EXISTS serie text;

-- Elimina claves foráneas previas si existen
ALTER TABLE portafolio_ticker DROP CONSTRAINT IF EXISTS portafolio_ticker_emisora_serie_fkey;
ALTER TABLE intradia_data DROP CONSTRAINT IF EXISTS intradia_data_emisora_serie_fkey;
ALTER TABLE estado_posicion DROP CONSTRAINT IF EXISTS estado_posicion_emisora_serie_fkey;
ALTER TABLE estado_resultado_trimestre DROP CONSTRAINT IF EXISTS estado_resultado_trimestre_emisora_serie_fkey;
ALTER TABLE estado_flujos DROP CONSTRAINT IF EXISTS estado_flujos_emisora_serie_fkey;
ALTER TABLE estado_resultado_acumulado DROP CONSTRAINT IF EXISTS estado_resultado_acumulado_emisora_serie_fkey;

-- Crea las claves foráneas correctas
ALTER TABLE portafolio_ticker
  ADD CONSTRAINT portafolio_ticker_emisora_serie_fkey
  FOREIGN KEY (emisoras, serie) REFERENCES emisoras (emisoras, serie);

ALTER TABLE intradia_data
  ADD CONSTRAINT intradia_data_emisora_serie_fkey
  FOREIGN KEY (emisoras, serie) REFERENCES emisoras (emisoras, serie);

ALTER TABLE estado_posicion
  ADD CONSTRAINT estado_posicion_emisora_serie_fkey
  FOREIGN KEY (emisoras, serie) REFERENCES emisoras (emisoras, serie);

ALTER TABLE estado_resultado_trimestre
  ADD CONSTRAINT estado_resultado_trimestre_emisora_serie_fkey
  FOREIGN KEY (emisoras, serie) REFERENCES emisoras (emisoras, serie);

ALTER TABLE estado_flujos
  ADD CONSTRAINT estado_flujos_emisora_serie_fkey
  FOREIGN KEY (emisoras, serie) REFERENCES emisoras (emisoras, serie);

ALTER TABLE estado_resultado_acumulado
  ADD CONSTRAINT estado_resultado_acumulado_emisora_serie_fkey
  FOREIGN KEY (emisoras, serie) REFERENCES emisoras (emisoras, serie);