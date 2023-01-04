/*
 * @file   tables.sql
 * @author Pratchaya Khansomboon (me@mononerv.dev)
 * @author Eric Lundin
 * @brief  Database tables for online store application.
 * @date   2022-12-09
 *
 * @copyright Copyright (c) 2022
 */
CREATE TABLE admin(
    id SERIAL NOT NULL UNIQUE,
    email VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(30) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE address(
    id SERIAL NOT NULL UNIQUE,
    street VARCHAR(30),
    postcode VARCHAR(8),
    city VARCHAR(30),
    country VARCHAR(30),
    telephone CHAR(10),
    PRIMARY KEY (id)
);

CREATE TABLE supplier(
    id SERIAL NOT NULL UNIQUE,
    admin_id INT NOT NULL,
    address_id INT NOT NULL UNIQUE,
    name VARCHAR(30) NOT NULL,
    description VARCHAR(512),
    orgnum VARCHAR(30) NOT NULL UNIQUE,
    PRIMARY KEY (id),
    FOREIGN KEY (admin_id) REFERENCES admin(id) on UPDATE CASCADE,
    FOREIGN KEY (address_id) REFERENCES address(id) on UPDATE CASCADE
);

CREATE TABLE customer(
    id SERIAL NOT NULL UNIQUE,
    address_id INT NOT NULL UNIQUE,
    first_name VARCHAR(30) NOT NULL,
    last_name VARCHAR(30) NOT NULL,
    email VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(30) NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (address_id) REFERENCES address(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE discount(
    id SERIAL NOT NULL UNIQUE,
    code VARCHAR(30) NOT NULL UNIQUE,
    name VARCHAR(30) NOT NULL,
    start_date DATETIME,
    end_date DATETIME,
    PRIMARY KEY(id)
);

CREATE TABLE product(
    id SERIAL NOT NULL UNIQUE,
    supplier_id INT NOT NULL,
    name VARCHAR(30) NOT NULL,
    description VARCHAR(1024),
    quantity INT NOT NULL,
    price NUMERIC(16,2) NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (supplier_id) REFERENCES supplier(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE cart(
    id SERIAL NOT NULL UNIQUE,
    customer_id INT NOT NULL UNIQUE,
    updated TIMESTAMP,
    PRIMARY KEY(id),
    FOREIGN KEY (customer_id) REFERENCES customer(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE orders(
    id SERIAL NOT NULL UNIQUE,
    customer_id INT NOT NULL,
    confirmed_by_admin INT NULL,
    created TIMESTAMP NOT NULL,
    status VARCHAR(30) NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (customer_id) REFERENCES customer(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (confirmed_by_admin) REFERENCES admin(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE discount_item(
    discount_id INT NOT NULL,
    product_id INT NOT NULL,
    factor NUMERIC(3, 2) NOT NULL, -- unsure about the data type
    PRIMARY KEY(discount_id, product_id),
    FOREIGN KEY(discount_id) REFERENCES discount(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES product(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE cart_item(
    cart_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    PRIMARY KEY(cart_id, product_id),
    FOREIGN KEY(cart_id) REFERENCES cart(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES product(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE order_item(
    id SERIAL NOT NULL UNIQUE,
    order_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    PRIMARY KEY(order_id, product_id),
    FOREIGN KEY(order_id) REFERENCES orders(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES product(id) ON UPDATE CASCADE ON DELETE CASCADE
);
