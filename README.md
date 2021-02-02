# sqlx postgres no index usage with wrong datatype 

## Setup

Start postgres:

```
docker run --rm -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:9.6.20
```

Setup database

```
./db_setup.sh
```

## Run program

```
env DATABASE_URL=postgres://postgres:password@localhost/sqlx cargo run --release
```

### Expected output
```
query with i32 takes 17.769324ms
query with u32 takes 222.777712ms
```
