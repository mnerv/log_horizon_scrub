CREATE TABLE address(
    id INT NOT NULL UNIQUE,
    street VARCHAR(30),
    city VARCHAR(30),
    country VARCHAR(30),
    telephone CHAR(10),
    PRIMARY KEY (id)
);

CREATE TABLE supplier(
    id INT NOT NULL UNIQUE,
    address_id INT NOT NULL,
    name VARCHAR(30),
    telephone CHAR(10),
    address VARCHAR(30),
    PRIMARY KEY (id),
    FOREIGN KEY (address_id) REFERENCES address(id) on UPDATE CASCADE
);

CREATE TABLE customer(
    id INT NOT NULL UNIQUE,
    address_id INT NOT NULL,
    firstname VARCHAR(30),
    lastname VARCHAR(30),
    email VARCHAR(30),
    password VARCHAR(30),
    PRIMARY KEY(id),
    FOREIGN KEY (address_id) REFERENCES address(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE product(
    id INT NOT NULL UNIQUE,
    supplier_id INT NOT NULL,
    quantity INT NOT NULL,
    nam VARCHAR(30),
    price NUMERIC(12,2),
    PRIMARY KEY(id),
    FOREIGN KEY (supplier_id) REFERENCES supplier(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE discount(
    id INT NOT NULL UNIQUE,
    code VARCHAR(30) NOT NULL,
    name VARCHAR(30) NOT NULL,
    start_date DATE,
    end_date DATE,
    PRIMARY KEY(id)
);

CREATE TABLE discount_product(
    discount_id INT NOT NULL,
    product_id INT NOT NULL,
    factor INT NOT NULL, -- unsure about the data type
    PRIMARY KEY(discount_id, product_id),
    FOREIGN KEY(discount_id) REFERENCES discount(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES product(id) ON UPDATE CASCADE ON DELETE CASCADE

CREATE TABLE shopping_list(
    id INT NOT NULL UNIQUE,
    customer_id INT NOT NULL,
    updated DATE,
    PRIMARY KEY(id),
    FOREIGN KEY (customer_id) REFERENCES customer(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE orders(
    id INT NOT NULL UNIQUE, 
    customer_id INT NOT NULL,
    created DATE NOT NULL,
    status VARCHAR(30) NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (customer_id) REFERENCES customer(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE shopping_cart(
    id INT NOT NULL UNIQUE,
    shopping_list_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY(shopping_list_id) REFERENCES shopping_list(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES product(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE order_detail(
    id INT NOT NULL UNIQUE,
    order_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    PRIMARY KEY(order_id, product_id),
    FOREIGN KEY(order_id) REFERENCES orders(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY(product_id) REFERENCES product(id) ON UPDATE CASCADE ON DELETE CASCADE
);
