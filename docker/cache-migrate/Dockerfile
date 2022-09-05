FROM debian:buster-slim as builder
RUN apt-get update && apt-get install -y git
WORKDIR /
RUN git clone https://github.com/vishnubob/wait-for-it.git

FROM redis:7.0.4
COPY --from=builder /wait-for-it/wait-for-it.sh /app/wait-for-it.sh
COPY /docker/cache-migrate/migrate.txt /etc/migrate.txt

ENTRYPOINT [\
  "/app/wait-for-it.sh", "cache:6379", "-s",\
  "--", "sh", "-c", "cat /etc/migrate.txt | redis-cli -h cache --pipe"\
]
