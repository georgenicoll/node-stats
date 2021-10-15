#!/bin/bash
CARGO_LOCATION=$(which cargo)
if [ ! -z "${CARGO_LOCATION}" ]; then
    echo "Running cargo clean"
    cargo clean || true
fi;

echo "Copying commands.json"
mkdir -p tmp
cp ../commands.json tmp/

echo "Running docker build followed by push"
docker build --tag georgenicoll/stats-backend-rust .
docker login
docker push georgenicoll/stats-backend-rust
