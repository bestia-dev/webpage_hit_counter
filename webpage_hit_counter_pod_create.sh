#!/usr/bin/env bash

# README:

echo " "
echo "\033[0;33m    Bash script to create the pod 'webpage_hit_counter_pod': 'sh webpage_hit_counter_pod_create.sh' \033[0m"
echo "\033[0;33m    Run this inside /var/www/transfer_folder/webpage_hit_counter \033[0m"
echo "\033[0;33m    This 'pod' is made of 2 containers: 'postgresql', 'webpage_hit_counter_cnt' \033[0m"
echo "\033[0;33m    It contains the webpage_hit_counter Rust web application and postgreSQL \033[0m"
echo "\033[0;33m    Published inbound network ports are 8011 and 5432 on 'localhost' \033[0m"

# repository: https://github.com/bestia-dev/webpage_hit_counter

# Start of script actions:

echo " "
echo "\033[0;33m    Create pod \033[0m"
# in a "pod" the "publish port" is tied to the pod and not containers.
# http connection     8011  (internally is 8080)
# postgres connection  5432

podman pod create \
-p 127.0.0.1:8011:8080/tcp \
-p 127.0.0.1:5432:5432/tcp \
--label name=webpage_hit_counter_pod \
--label version=1.0 \
--label source=github.com/bestia-dev/webpage_hit_counter \
--label author=github.com/bestia-dev \
--name webpage_hit_counter_pod

echo " "
echo "\033[0;33m    Create container webpage_hit_counter_cnt in the pod \033[0m"
podman create --name webpage_hit_counter_cnt \
--pod=webpage_hit_counter_pod -ti \
webpage_hit_counter_img

echo " "
echo "\033[0;33m    Create container postgresql in the pod \033[0m"
podman pull docker.io/library/postgres:13

podman run --name postgresql --pod=webpage_hit_counter_pod -d \
  -e POSTGRES_USER=admin \
  -e POSTGRES_PASSWORD=Passw0rd \
  docker.io/library/postgres:13

echo "\033[0;33m    podman pod start \033[0m"
podman pod start webpage_hit_counter_pod

echo " "
echo "\033[0;33m    You can administer your postgreSQL in psql on: \033[0m"
echo "\033[0;33m localhost:5432 \033[0m"

echo " "
echo "\033[0;33m    You can delete the pod and ALL of the DATA it contains: \033[0m"
echo "\033[0;33m podman pod rm -f webpage_hit_counter_pod \033[0m"