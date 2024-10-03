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
docker kill $(docker ps -q) && docker rm $(docker ps -a -q)
```

Add a user:

```bash
curl -X POST "http://localhost:8000/subscriptions" \
    --data-urlencode "email=a.b@c.com" \
    --data-urlencode "name=eddy"
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