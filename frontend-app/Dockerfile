FROM node as build

COPY src /src
COPY static /static
COPY *.cjs /
COPY *.js /
COPY *.json /
RUN npm update && npm run build

FROM node as runtime
COPY --from=build build stats-frontend/app/
COPY --from=build package-cutdown.json stats-frontend/package.json

WORKDIR /stats-frontend
ENTRYPOINT [ "node", "app/index.js" ]
