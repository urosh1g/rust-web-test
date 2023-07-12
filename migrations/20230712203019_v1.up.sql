alter table if exists articles 
alter column author_id set not null;

alter table if exists articles
alter column content set not null;

alter table if exists comments
alter column user_id set not null;

alter table if exists comments
alter column article_id set not null;
