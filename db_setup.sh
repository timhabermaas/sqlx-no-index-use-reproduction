#! /bin/sh

psql -c "drop database sqlx" "user=postgres host=localhost dbname=postgres password=password"
psql -c "create database sqlx" "user=postgres host=localhost dbname=postgres password=password"
psql -c "CREATE TABLE tweet(id BIGSERIAL PRIMARY KEY, created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(), text TEXT NOT NULL, owner_id BIGINT);" "user=postgres host=localhost dbname=sqlx password=password"
psql -c "INSERT INTO tweet(text, owner_id) SELECT 'some text', random() * 1000 from generate_series(1, 1000000);" "user=postgres host=localhost dbname=sqlx password=password"
psql -c "CREATE INDEX owner_id_on_tweet ON tweet (owner_id);" "user=postgres host=localhost dbname=sqlx password=password"
