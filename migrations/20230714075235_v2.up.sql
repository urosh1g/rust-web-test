alter table if exists users
rename column id to user_id;

alter table if exists articles
rename column id to article_id;

alter table if exists comments
rename column id to comment_id;

drop table if exists likes;
create table if not exists likes (
	like_id serial primary key,
	user_id integer references users(user_id) not null,
	article_id integer references articles(article_id) not null
);
