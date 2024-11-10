#!/bin/sh

cargo install sea-orm-cli

sea-orm-cli generate entity \
    -u postgres://rust:rust@localhost:5432/rust \
    -o src/entities
