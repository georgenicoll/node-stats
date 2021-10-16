#!/bin/bash
docker run \
  --publish 3000:3000 \
  --env BACKEND_DNS_ADDRESS=mandrill.lan \
  --env BACKEND_PORT=9000 \
  --workdir /stats-frontend \
  georgenicoll/stats-frontend \
  node app/index.js
