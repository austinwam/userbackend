<!-- https://dev.to/behainguyen/rust-sqlx-cli-database-migration-with-mysql-and-postgresql-42gp -->

cargo install sqlx-cli

### db url

let db_url = "postgresql://postgres:password@localhost:5432/userdb";

## create migration

sqlx database create --database-url "postgresql://postgres:password@localhost:5432/userdb"
sqlx migrate add 1
sqlx migrate run --database-url "postgresql://postgres:password@localhost:5432/userdb"
