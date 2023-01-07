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
