FROM debian:buster-slim as builder
RUN apt-get update && apt-get install -y git curl
WORKDIR /
RUN git clone https://github.com/vishnubob/wait-for-it.git
RUN curl -L -H "Accept: application/zip" https://github.com/karatelabs/karate/releases/download/v1.2.1.RC1/karate-1.2.1.RC1.jar -o karate.jar 

FROM eclipse-temurin:11
RUN mkdir /app
COPY --from=builder /wait-for-it/wait-for-it.sh /app/wait-for-it.sh
COPY --from=builder /karate.jar /app

CMD [\
  "/app/wait-for-it.sh", "service:8080", "-s", "-t", "30", "&&",\
  "echo", "'Running tests!'", "&&",\
  "java", "-jar", "-Dkarate.config.dir=\"docker/karate\"", "/app/karate.jar", "docker/karate/features", "-e", "docker", "-o", "/app/target", "--tags", "~@ignore"\
]
