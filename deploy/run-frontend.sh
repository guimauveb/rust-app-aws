#!/bin/bash
kill "$(<~/pids/rust-app-frontend)" &&
cd ~/rust-app/frontend &&
printf "API_URL=http://api.your-domain.com" > .env
TRUNK_BUILD_RELEASE=true
trunk serve > ~/logs/rust-app-frontend 2>~/logs/rust-app-frontend & echo $! > ~/pids/rust-app-frontend
