create table roles (
    id int primary key,
    name varchar (50) not null
);

insert into roles values (1, 'Покупатель');
insert into roles values (2, 'Менеджер');
insert into roles values (3, 'Админ');


create table users (
    id serial primary key,
    email varchar(100) not null unique,
    password varchar(100) not null,
    name varchar(300) null null,
    role int references roles (id)
);