create table if not exists users (
        id serial primary key,
        email varchar(20) not null unique,
        password varchar(20) not null
);

create table if not exists articles (
        id serial primary key,
        author_id integer references users(id) on delete cascade,
        title varchar(20) not null,
        content text
);

create table if not exists comments (
        id serial primary key,
        user_id integer references users(id) on delete cascade,
        article_id integer references articles(id) on delete cascade,
        content text not null
);

create table if not exists likes (
	user_id integer references users(id) on delete cascade,
	article_id integer references articles(id) on delete cascade,
	primary key (user_id, article_id)
);
