-- Enable the uuid-ossp extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "items" (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(255) NOT NULL,
    base_price  DECIMAL(10, 2) NOT NULL,
    description TEXT NOT NULL,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "users" (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nickname   VARCHAR(255) NOT NULL,
    email      VARCHAR(255) NOT NULL,
    password   VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE "markets" (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        VARCHAR(255) NOT NULL,
    description TEXT NULL,
    type        VARCHAR(8) CHECK (type IN ('public', 'private')) NOT NULL,
    tax_percent DECIMAL(5, 2) DEFAULT 0,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "orders" (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type        VARCHAR(5) CHECK (type IN ('buy', 'sell')) NOT NULL,
    price       DECIMAL(10, 2),
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "slot" (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "customers" (
     id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
     balance    DECIMAL(10, 2) DEFAULT 100.00,
     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "markets_customers_rel" (
    market_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    CONSTRAINT fk_markets_customers_rel_market_id FOREIGN KEY (market_id) REFERENCES "markets"(id) ON DELETE CASCADE,
    CONSTRAINT fk_markets_customers_rel_customer_id FOREIGN KEY (customer_id) REFERENCES "customers"(id) ON DELETE CASCADE
);