-- Your SQL goes here
CREATE TABLE variants (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE products_variants (
    id SERIAL PRIMARY KEY,
    variant_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    value VARCHAR,
    FOREIGN KEY(variant_id) REFERENCES variants(id),
    FOREIGN KEY(product_id) REFERENCES products(id)
);
