#!/bin/bash
docker run \
  --rm \
  --publish 3000:3456 \
  --env PORT=3456 \
  --env BACKEND_DNS_ADDRESS=mandrill.lan \
  --env BACKEND_PORT=9000 \
  georgenicoll/stats-frontend
