\set ON_ERROR_STOP on
CREATE DATABASE verbena TEMPLATE template0 ENCODING 'UTF8' LOCALE 'en_US.UTF-8';
\c verbena
CREATE TABLE country(
	code TEXT PRIMARY KEY,
	name TEXT
);
CREATE TABLE customer(
	id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name TEXT,
	email TEXT,
	phone TEXT,
	delivery_address TEXT,
	billing_address TEXT
);
CREATE TABLE order(
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	customer INTEGER NOT NULL REFERENCES customer(id),
	date DATE NOT NULL,
	due DATE
);
CREATE TABLE product(
	id TEXT PRIMARY KEY,
	name TEXT,
	cost DECIMAL,
	price DECIMAL
);
CREATE TABLE order_line(
	id BIGINT NOT NULL REFERENCES order(id),
	line SMALLINT NOT NULL,
	product TEXT REFERENCES product(id),
	description TEXT,
	qty DECIMAL,
	price DECIMAL
);
