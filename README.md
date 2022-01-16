# rosu.

A fast, efficient emulator for the osu! Bancho protocol written in Rust.

## Setup

Git clone rosu, setup your nginx (example config is in the ext folder).

Use `cargo build` to build, if you are not in the need of debugging I recommend building with `cargo build --release` for optimisations that will speed up rosu by 3-5x. ROsu will be built in `target/debug/rosu` if not using release, and `target/release/rosu` otherwise.

ROsu uses MySQL for it's database and uses a database DSN with the format:

`mysql://username:password@host/database`

For security and ease of use, you set this url as the environment variable `DATABASE_URL` and ROsu will use that for connections. An example DB schema for ROsu is provided in the `ext` folder also.

## Features

Currently, logins are 100% functional and we are beginning to handle packets that are sent to the server post login (action updates etc.)