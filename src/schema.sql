\set ON_ERROR_STOP true
CREATE DATABASE verbena TEMPLATE template0 ENCODING 'UTF8' LOCALE 'en_US.UTF-8';
\c verbena
BEGIN;
CREATE TABLE country(
	code TEXT PRIMARY KEY,
	name TEXT  not null
);
CREATE TABLE address(
	id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name TEXT not null,
	address_1 TEXT ,
	address_2 TEXT,
	city TEXT,
	region TEXT,
	postal_code TEXT,
	country TEXT NOT NULL references country(code)
);
CREATE TABLE customer(
	id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	name TEXT NOT  NULL,
	email TEXT,
	phone TEXT,
	bill_to integer not null references address(id),
	deliver_to integer not null references address(id)
);
CREATE TABLE "order"(
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	customer INTEGER NOT NULL REFERENCES customer(id),
	bill_to integer not null references address(id),
	deliver_to integer not null references address(id),
	date DATE NOT NULL,
	due DATE
);
CREATE TABLE product(
	id TEXT PRIMARY KEY,
	name TEXT NOT  NULL,
	cost DECIMAL,
	price DECIMAL
);
CREATE TABLE order_line(
	id BIGINT NOT NULL REFERENCES "order"(id),
	line SMALLINT NOT NULL,
	product TEXT not null REFERENCES product(id),
	description TEXT,
	qty DECIMAL not null,
	price DECIMAL not null
);
COMMIT;
