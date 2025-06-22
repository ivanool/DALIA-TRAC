
-- Tabla de portafolios (cada usuario puede tener varios)

-- Asegúrate de que la tabla emisoras tenga UNIQUE en la columna emisoras
-- Si ya existe la tabla, ejecuta esto aparte:
-- ALTER TABLE emisoras ADD CONSTRAINT emisoras_unique UNIQUE (emisoras);

-- Relaciona cada ticker con un portafolio;
CREATE TABLE portafolio_ticker (
    id SERIAL PRIMARY KEY,
    portafolio_id INTEGER REFERENCES portafolios(id) ON DELETE CASCADE,
    ticker TEXT NOT NULL REFERENCES emisoras(emisoras)
);

-- Transacciones ligadas al portafolio y ticker
CREATE TABLE transacciones (
    id SERIAL PRIMARY KEY,
    portafolio_ticker_id INTEGER REFERENCES portafolio_ticker(id) ON DELETE CASCADE,
    tipo TEXT NOT NULL, -- 'compra' o 'venta'
    cantidad INTEGER NOT NULL,
    precio DECIMAL NOT NULL,
    fecha TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Movimientos de caja ligados al portafolio
CREATE TABLE cashflow (
    id SERIAL PRIMARY KEY,
    portafolio_id INTEGER REFERENCES portafolios(id) ON DELETE CASCADE,
    monto DECIMAL NOT NULL,
    tipo TEXT NOT NULL, -- 'entrada' o 'salida'
    fecha TIMESTAMP NOT NULL DEFAULT NOW(),
    descripcion TEXT
);

-- Dividendos ligados al portafolio y ticker
CREATE TABLE dividendos (
    id SERIAL PRIMARY KEY,
    portafolio_ticker_id INTEGER REFERENCES portafolio_ticker(id) ON DELETE CASCADE,
    monto DECIMAL NOT NULL,
    fecha TIMESTAMP NOT NULL DEFAULT NOW()
);