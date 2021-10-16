#!/bin/bash
NPM_LOCATION=$(which npm)
if [ ! -z "${NPM_LOCATION}" ]; then
    echo "Running npm update"
    npm install clean || true
fi;

echo "Running docker build followed by push"
docker build --tag georgenicoll/stats-frontend .
docker login
docker push georgenicoll/stats-frontend
