FROM debian:buster-slim as builder
RUN apt-get update && apt-get install -y curl
WORKDIR /
RUN curl -L -H "Accept: application/zip" https://github.com/karatelabs/karate/releases/download/v1.2.1.RC1/karate-1.2.1.RC1.jar -o karate.jar 

FROM eclipse-temurin:11
RUN mkdir /app
COPY --from=builder /karate.jar /app
CMD [\
  "java", "-jar", "/app/karate.jar",\
  "-p", "8181",\
  "-m", "/app/mocks/MockFeature.feature"\
]
