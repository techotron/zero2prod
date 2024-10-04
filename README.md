Start the DB:

```bash
scripts/init_db.sh
```

Run the server:

```bash
cargo run
```

Note: run with log level, set the `RUST_LOG` env var

Stop the DB:

```bash
docker kill $(docker ps -q) && docker rm $(docker ps -a -q)
```

Run Tests:
```bash
scripts/init_db.sh
cargo test
# TEST_LOG=true cargo test # to see the log output from the tests
docker kill $(docker ps -q) && docker rm $(docker ps -a -q)
```

Add a user:

```bash
curl -X POST "http://localhost:8000/subscriptions" \
    --data-urlencode "email=a.b@c.com" \
    --data-urlencode "name=eddy"
```

Linting
```bash
cargo clippy -- -D warnings
cargo fmt --all
```

Remove unused dependencies:

```bash
cargo install cargo-udeps
#rustup toolchain install nightly
cargo +nightly udeps

# example (partial) output
unused dependencies:
`zero2prod v0.1.0 (/Users/edward.snow/git/github.com/techotron/zero2prod)`
└─── dependencies
     └─── "env_logger"
```

Connect to the DB
```bash
psql -h localhost -p 5432 -U postgres -d newsletter
```

Prepare build with sqlx offline
```bash
SQLX_OFFLINE=true cargo prepare -- --bin zero2prod
```