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
echo "\033[0;33m    Removing pod if exists \033[0m"
# Be careful, this containers are not meant to have persistent data.
# the '|| :' in combination with 'set -e' means that 
# the error is ignored if the container does not exist.
set -e
podman pod rm -f webpage_hit_counter_pod || :

echo " "
echo "\033[0;33m    Create pod \033[0m"
# in a "pod" the "publish port" is tied to the pod and not containers.
# http connection     8011  (forwarding from internal port 8080)
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
echo "\033[0;33m    Create container postgresql in the pod \033[0m"
podman pull docker.io/library/postgres:13

echo "\033[0;33m    Volume: /home/luciano_bestia/postgres_data/webpage_hit_counter_pod \033[0m"
mkdir -p /home/luciano_bestia/postgres_data/webpage_hit_counter_pod

podman run --name postgresql --pod=webpage_hit_counter_pod -d \
  -e POSTGRES_USER=admin \
  -e POSTGRES_PASSWORD=Passw0rd \
  -v /home/luciano_bestia/postgres_data/webpage_hit_counter_pod:/var/lib/postgresql/data \
  docker.io/library/postgres:13

echo " "
echo "\033[0;33m    Create container webpage_hit_counter_cnt in the pod and at last run the web app \033[0m"
podman create --name webpage_hit_counter_cnt \
--pod=webpage_hit_counter_pod -ti \
-w /home/rustdevuser/rustprojects/webpage_hit_counter \
webpage_hit_counter_img:2022-08-09 \
./webpage_hit_counter

echo "\033[0;33m    Wait 5 seconds, so the connection to postgres is not refused. \033[0m"
sleep 5

echo "\033[0;33m    podman pod start \033[0m"
podman pod start webpage_hit_counter_pod

echo " "
echo "\033[0;33m    You can administer your postgreSQL in psql with: \033[0m"
echo "\033[0;33m psql -h localhost -p 5432 -U admin -W -d webpage_hit_counter \033[0m"

echo " "
echo "\033[0;33m    You can check the application std output with: \033[0m"
echo "\033[0;33m podman logs webpage_hit_counter_cnt \033[0m"

echo " "
echo "\033[0;33m    Test the web application locally: \033[0m"
echo "\033[0;33m curl http://localhost:8011/webpage_hit_counter/get_svg_image/555555.svg \033[0m"

echo " "
echo "\033[0;33m    Test the web application on the internet: \033[0m"
echo "\033[0;33m curl https://bestia.dev/webpage_hit_counter/get_svg_image/555555.svg \033[0m"
    
echo " "
echo "\033[0;33m    You can delete the pod. The SQL data is persistent on the system disk: \033[0m"
echo "\033[0;33m podman pod rm -f webpage_hit_counter_pod \033[0m"