FROM alpine:edge as build

RUN apk upgrade --no-cache && \
    apk add nodejs-current npm
COPY src /src
COPY static /static
COPY *.cjs /
COPY *.js /
COPY *.json /
RUN npm update && npm run build

FROM alpine:edge as runtime
RUN apk upgrade --no-cache && \
    apk add bash nodejs-current
COPY --from=build build stats-frontend/app/
COPY --from=build package-cutdown.json stats-frontend/package.json
