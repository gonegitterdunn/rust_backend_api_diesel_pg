# Demo Actix-Web Rest API Project

### In order to run:

- Install [postgresql](https://www.postgresql.org/download)
- Install [diesel](https://diesel.rs/guides/getting-started.html)
- Install [docker](https://docs.docker.com/get-docker/)
- Install [postman](https://www.postman.com/downloads/)
- Install cargo-watch -> `cargo install systemfd cargo-watch`

### To run locally:

- Start docker
- Run `bash scripts/init_script.sh`
- Run `systemfd --no-pid -s http::5000 -- cargo watch -x run`
