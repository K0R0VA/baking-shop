create table product_types (
    id serial primary key,
    name varchar(100) not null
);

create table products (
  id serial primary key,
  name varchar(150) not null,
  pic varchar(150) not null,
  price varchar(150),
  type int references product_types (id),
  weight int,
  description text
);