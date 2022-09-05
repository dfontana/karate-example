#!/bin/bash
/app/wait-for-it.sh service:8080 -s -t 30
echo "Running tests!"
java -jar -Dkarate.config.dir="docker/karate" /app/karate.jar docker/karate/features -e docker -o /app/target --tags ~@ignore
