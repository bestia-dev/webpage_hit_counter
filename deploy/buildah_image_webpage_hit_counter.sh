#!/usr/bin/env bash

# README:

echo " "
echo "\033[0;33m    Bash script to build the docker image for webpage_hit_counter. \033[0m"
echo "\033[0;33m    Name of the image: webpage_hit_counter_img \033[0m"
# repository: https://github.com/bestia-dev/webpage_hit_counter

echo " "
echo "\033[0;33m    To build the image, run in bash with: \033[0m"
echo "\033[0;33m sh buildah_image.sh \033[0m"

# Start of script actions:

echo " "
echo "\033[0;33m    Removing container and image if exists \033[0m"
# Be careful, this container is not meant to have persistent data.
# the '|| :' in combination with 'set -e' means that 
# the error is ignored if the container does not exist.
set -e
podman rm webpage_hit_counter_cnt || :
buildah rm webpage_hit_counter_img || :
buildah rmi -f webpage_hit_counter_img || :

echo " "
echo "\033[0;33m    Create new 'buildah container' named webpage_hit_counter_img \033[0m"
set -o errexit

buildah from --name webpage_hit_counter_img docker.io/library/debian:bullseye-slim

buildah config \
--author=github.com/bestia-dev \
--label name=webpage_hit_counter_img \
--label version=2022-08-09 \
--label source=github.com/bestia-dev/webpage_hit_counter \
webpage_hit_counter_img

echo " "
echo "\033[0;33m    apk update \033[0m"
buildah run webpage_hit_counter_img    apt -y update
buildah run webpage_hit_counter_img    apt -y full-upgrade

echo " "
echo "\033[0;33m    Create non-root user 'rustdevuser' and home folder. \033[0m"
buildah run webpage_hit_counter_img    useradd -ms /bin/bash rustdevuser

echo " "
echo "\033[0;33m    Use rustdevuser for all subsequent commands. \033[0m"
buildah config --user rustdevuser webpage_hit_counter_img
buildah config --workingdir /home/rustdevuser webpage_hit_counter_img

# If needed, the user can be forced for a buildah command:
# buildah run  --user root webpage_hit_counter_img    apt install -y --no-install-recommends build-essential

echo " "
echo "\033[0;33m    Configure rustdevuser things \033[0m"
buildah run webpage_hit_counter_img /bin/sh -c 'mkdir -vp ~/rustprojects'
buildah run webpage_hit_counter_img /bin/sh -c 'mkdir -vp ~/rustprojects/webpage_hit_counter'

echo " "
echo "\033[0;33m    Kill auto-completion horrible sound \033[0m"
buildah run webpage_hit_counter_img /bin/sh -c 'echo "set bell-style none" >> ~/.inputrc'

echo " "
echo "\033[0;33m    Copy the binary and make it executable. The owner is rustdevuser (1000)  \033[0m"
buildah copy --chown 1000:1000 webpage_hit_counter_img './webpage_hit_counter' '/home/rustdevuser/rustprojects/webpage_hit_counter'
buildah run webpage_hit_counter_img /bin/sh -c 'chmod 755 /home/rustdevuser/rustprojects/webpage_hit_counter'
buildah run webpage_hit_counter_img /bin/sh -c 'ls -la /home/rustdevuser/rustprojects/webpage_hit_counter'
buildah copy --chown 1000:1000 webpage_hit_counter_img './.env' '/home/rustdevuser/rustprojects/webpage_hit_counter'

echo " "
echo "\033[0;33m    Remove unwanted files \033[0m"
buildah run --user root webpage_hit_counter_img    apt -y autoremove
buildah run --user root webpage_hit_counter_img    apt -y clean

echo " "
echo "\033[0;33m    Finally save/commit the image named webpage_hit_counter_img \033[0m"
buildah commit webpage_hit_counter_img webpage_hit_counter_img:2022-08-09

echo " "
echo "\033[0;33m    This image will be used in the pod webpage_hit_counter_pod with postgres. \033[0m"
echo "\033[0;33m    run the bash script 'sh webpage_hit_counter_pod_create.sh' \033[0m"
