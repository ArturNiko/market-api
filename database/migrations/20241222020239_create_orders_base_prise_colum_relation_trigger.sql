-- if order price doesn't exist or is lower than item base price, set item base price
CREATE OR REPLACE FUNCTION set_order_price()
    RETURNS TRIGGER AS $$
DECLARE
    item_base_price DECIMAL;
BEGIN
     -- Select the base price from the "items" table based on the "item_id" and assign it to "item_base_price" declared variable
    SELECT "base_price" INTO item_base_price
    FROM "items"
    WHERE "items"."id" = NEW."item_id"
        LIMIT 1;

    -- Check if the price is NULL or lower than the base price
    IF NEW."price" IS NULL OR NEW."price" < item_base_price THEN
        SELECT "base_price" INTO NEW."price"
        FROM "items"
        WHERE "items"."id" = NEW."item_id"
            LIMIT 1;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER orders_set_price
    BEFORE INSERT ON "orders"
    FOR EACH ROW
    EXECUTE FUNCTION set_order_price();
