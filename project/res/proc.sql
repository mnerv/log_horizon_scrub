/*
 * @file   proc.sql
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Database procedures for data insertions.
 * @date   2023-01-04
 *
 * @copyright Copyright (c) 2023
 */
CREATE OR REPLACE PROCEDURE insert_customer(
    first_name VARCHAR,
    last_name VARCHAR,
    email VARCHAR,
    pass VARCHAR,
    street VARCHAR,
    postcode VARCHAR,
    city VARCHAR,
    country VARCHAR,
    tele VARCHAR
)
LANGUAGE plpgsql
AS $$
DECLARE
    address_id INT;
BEGIN
    INSERT INTO address(street, postcode, city, country, telephone)
    VALUES (street, postcode, city, country, tele)
    RETURNING id INTO address_id;

    INSERT INTO customer (address_id, first_name, last_name, email, password)
    VALUES (address_id, first_name, last_name, email, pass);
END;
$$;

CREATE OR REPLACE PROCEDURE insert_supplier(
    admin_id INT,
    name VARCHAR,
    description VARCHAR,
    orgnum VARCHAR,
    street VARCHAR,
    postcode VARCHAR,
    city VARCHAR,
    country VARCHAR,
    tele VARCHAR
)
LANGUAGE plpgsql
AS $$
DECLARE
    address_id INT;
BEGIN
    INSERT INTO address(street, postcode, city, country, telephone)
    VALUES (street, postcode, city, country, tele)
    RETURNING id INTO address_id;

    INSERT INTO supplier (admin_id ,address_id, name, description, orgnum)
    VALUES (admin_id, address_id, name, description, orgnum);
END;
$$;

CREATE OR REPLACE PROCEDURE add_to_cart(
    customer_id INT,
    product_id INT,
    quantity INT
)
LANGUAGE plpgsql
AS $$
DECLARE
    current_cart_id INT;
    product_quantity INT;
    item_quantity INT;
BEGIN
    SELECT id INTO current_cart_id FROM cart WHERE add_to_cart.customer_id = cart.customer_id;
    IF current_cart_id IS NULL THEN
        INSERT INTO cart(customer_id, updated) VALUES (customer_id, CURRENT_TIMESTAMP)
        RETURNING id INTO current_cart_id;
    END IF;

    SELECT product.quantity INTO product_quantity FROM product WHERE id = product_id;
    SELECT cart_item.quantity INTO item_quantity FROM cart_item
        WHERE cart_item.cart_id = current_cart_id AND cart_item.product_id = add_to_cart.product_id;

    -- Jank ass IF statement
    IF product_quantity >= add_to_cart.quantity + item_quantity THEN
        IF item_quantity = 0 THEN
            INSERT INTO cart_item (cart_id, product_id, quantity)
            VALUES (current_cart_id, product_id, quantity);
        END IF;
        IF item_quantity > 0 THEN
            UPDATE cart_item SET quantity = add_to_cart.quantity + item_quantity
                WHERE cart_item.cart_id = cart_id AND cart_item.product_id = add_to_cart.product_id;
        END IF;

        UPDATE cart SET updated = CURRENT_TIMESTAMP WHERE id = current_cart_id;
    END IF;
END;
$$;

CREATE OR REPLACE PROCEDURE checkout(customer_id INT)
LANGUAGE plpgsql
AS $$
DECLARE
    current_cart_id INT;
    order_id INT;
BEGIN
    SELECT id INTO current_cart_id FROM cart WHERE cart.customer_id = checkout.customer_id;
    IF current_cart_id IS NULL THEN
        RAISE EXCEPTION 'No cart found for customer %', customer_id;
    END IF;

    INSERT INTO orders (customer_id, created, status) VALUES (customer_id, CURRENT_TIMESTAMP, 'unconfirmed')
    RETURNING id INTO order_id;

    INSERT INTO order_item (order_id, product_id, quantity)
    SELECT ci.cart_id, ci.product_id, ci.quantity FROM cart_item AS ci WHERE ci.cart_id = current_cart_id;

    UPDATE product SET quantity = quantity - cart_item.quantity FROM cart_item
    WHERE product.id = cart_item.product_id AND cart_item.cart_id = current_cart_id;

    DELETE FROM cart_item WHERE cart_item.cart_id = current_cart_id;
END;
$$;
