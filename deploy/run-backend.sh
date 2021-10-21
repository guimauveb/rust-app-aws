#!/bin/bash
kill "$(<~/pids/rust-app-backend)" &&
cd ~/rust-app/backend &&
printf "DATABASE_URL=postgres://rust_app:rust_app@localhost/rust_app\nAPI_URL=http://api.your-domain.com" > .env
cd ~/rust-app/backend &&
cargo build --release > ~/logs/rust-app-backend 2>&1 && (./target/release/rust-app-backend > ~/logs/rust-app-backend 2>&1 & echo $! > ~/pids/rust-app-backend)
