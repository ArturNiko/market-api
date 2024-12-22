
-- Trigger to set 'price' from 'items.base_price' if price is not provided
CREATE OR REPLACE FUNCTION set_order_price()
RETURNS TRIGGER AS $$
BEGIN
  -- If price is NULL, fetch the base_price from 'items' table
  IF NEW.price IS NULL THEN
SELECT base_price INTO NEW.price
FROM items
WHERE items.id = NEW.user_id  -- assuming user_id relates to items in some way
    LIMIT 1;
END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER orders_set_price
    BEFORE INSERT ON orders
    FOR EACH ROW
    EXECUTE FUNCTION set_order_price();