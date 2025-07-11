-- Elimina la tabla vieja si existe para evitar conflictos
DROP TABLE IF EXISTS portfolio_transactions;
DROP TABLE IF EXISTS portfolio_cash;

-- Tabla de transacciones del portafolio
CREATE TABLE IF NOT EXISTS portfolio_transactions (
    transaction_id SERIAL PRIMARY KEY,
    portfolio_id INTEGER NOT NULL,                        -- FK al portafolio
    user_id INTEGER NOT NULL,                             -- FK redundante para consultas rápidas
    ticker VARCHAR(20) NOT NULL,
    transaction_type VARCHAR(12) NOT NULL CHECK (transaction_type IN ('BUY', 'SELL', 'DIVIDEND', 'DEPOSIT', 'WITHDRAWAL')),
    quantity DECIMAL(18, 8) NOT NULL,
    price DECIMAL(18, 4),
    transaction_date TIMESTAMPTZ NOT NULL,
    total_amount DECIMAL(18, 4) NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'MXN',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_portfolio FOREIGN KEY (portfolio_id) REFERENCES portafolios(id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

-- Tabla para gestionar el efectivo dentro del portafolio
CREATE TABLE IF NOT EXISTS portfolio_cash (
    cash_id SERIAL PRIMARY KEY,
    portfolio_id INTEGER NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    balance DECIMAL(18, 4) NOT NULL DEFAULT 0.00,
    currency VARCHAR(10) NOT NULL DEFAULT 'MXN',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT fk_portfolio_cash FOREIGN KEY (portfolio_id) REFERENCES portafolios(id) ON DELETE CASCADE,
    CONSTRAINT fk_user_cash FOREIGN KEY (user_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

-- Índices para acelerar las consultas
CREATE INDEX IF NOT EXISTS idx_transactions_portfolio_ticker ON portfolio_transactions (portfolio_id, ticker);
CREATE INDEX IF NOT EXISTS idx_transactions_user_date ON portfolio_transactions (user_id, transaction_date);
