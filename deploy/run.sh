#!/bin/bash
cd ~/rust-app/deploy &&
git pull > ~/logs/rust-app-git-pull 2>&1 &
./run-backend.sh &&
./run-frontend.sh

