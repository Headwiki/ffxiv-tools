CREATE TABLE IF NOT EXISTS item (
	item_id serial PRIMARY KEY,
	name VARCHAR (128) UNIQUE NOT NULL,
	image VARCHAR (32) NULL,
	is_untradeable boolean NOT NULL
);