#!/bin/bash
docker run -it -p 9000:9000 \
  --volume=/sys/module/thermal:/sys/module/thermal \
  --volume=/etc/hostname:/etc/hostname \
  georgenicoll/stats-backend-rust /bin/bash 