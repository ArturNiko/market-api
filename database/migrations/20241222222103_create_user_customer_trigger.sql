CREATE FUNCTION create_user_customer_trigger()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO "customers" ("user_id")
    VALUES (0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER user_customer_trigger
    AFTER INSERT ON "users"
    FOR EACH ROW
    EXECUTE FUNCTION create_user_customer_trigger();