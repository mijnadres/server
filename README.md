# server
mijnadres server, responding to all sort of requests.

## Problem
This project aims to solve the following problem. Over time people are
interested where you live. When you move you have to let them know where to find
you. What if we could change that dependency?

So instead you keeping track who is interested where you live and being
responsible for updating their information, people who are interested in your
address get notified when you update it.

We aim to have a single source of truth that clients can get a part of, ensuring
that the are notified when changes are made.

## Development
The server is developed in [rust][]. Furthermore we make use of [docker][] to provide a containered Postgres database.

To run the Postgres database run the following command

```shell
docker run --name test-postgres -p5432:5432 -e POSTGRES_PASSWORD=postgres -d postgres:9.6.5
```

We use the [`diesel`][diesel] ORM. It comes with an commandline tool that helps with development. Install it with

```shell
cargo install diesel_cli --no-default-features --features postgres
```

This only installs the postgres features. Change the `.env` file to reflect the correct `DATABASE_URL` and run

```shell
diesel migration run
```

This migrates the database to the latest state. The next command will start the server.

```shell
cargo run --bin server
```

[rust]: https://www.rust-lang.org/en-US/
[docker]: https://www.docker.com/
[diesel]: http://diesel.rs/