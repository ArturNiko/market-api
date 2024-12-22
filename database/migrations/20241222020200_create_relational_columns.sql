ALTER TABLE "customers"
    ADD COLUMN user_id UUID NOT NULL,
    ADD COLUMN market_id UUID NOT NULL,
    ADD CONSTRAINT fk_customers_user_id FOREIGN KEY (user_id) REFERENCES "users"(id) ON DELETE CASCADE,
    ADD CONSTRAINT fk_customers_market_id FOREIGN KEY (market_id) REFERENCES "markets"(id) ON DELETE CASCADE;

ALTER TABLE "items"
    ADD COLUMN owner_id UUID NOT NULL,
    ADD COLUMN offer_id UUID DEFAULT NULL,
    ADD CONSTRAINT fk_items_owner_id FOREIGN KEY (owner_id) REFERENCES "customers"(id) ON DELETE CASCADE,
    ADD CONSTRAINT fk_items_offer_id FOREIGN KEY (offer_id) REFERENCES "orders"(id) ON DELETE SET NULL;

ALTER TABLE "orders"
    ADD COLUMN market_id UUID NOT NULL,
    ADD CONSTRAINT fk_orders_market_id FOREIGN KEY (market_id) REFERENCES "markets"(id) ON DELETE CASCADE;


ALTER TABLE "markets"
    ADD COLUMN owner_id UUID NOT NULL,
    ADD CONSTRAINT fk_markets_owner_id FOREIGN KEY (owner_id) REFERENCES "customers"(id) ON DELETE CASCADE;