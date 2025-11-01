#!/bin/sh
# webpage_hit_counter/ssh_psql_insert_webpage.sh

printf " \n"
printf "\033[0;33m    Bash script to insert a webpage in the webpage_hit_counter database \033[0m\n"
printf "\033[0;33m    on bestia.dev over SSH in one command. \033[0m\n"
printf "\033[0;33m    Use sshadd to enter the passphrase for the SSH connection beforehand. \033[0m\n"
printf "\033[0;33m    The bash script needs 1 arguments: host_name like this: \033[0m\n"
printf "\033[0;33m    sh ssh_psql_insert_webpage.sh test.com \033[0m\n"
# repository: https://github.com/bestia-dev/webpage_hit_counter

if [ $# != 1 ]
then
  printf " \n"
  printf "\033[0;31m Incorrect number of arguments. The only argument is host_name like github.com or bestia.dev \033[0m\n"
  printf " \n"
  exit 1
fi

# psql asks a password prompt in plain text. Everybody watching the screen can see the password.
# I will ask for the password separately and store it in the variable $password.
# Then I will send the command to store it as env var PGPASSWORD on the server. It will live only for this session.
printf "Postgres Server Password: "
read -s password

printf " \n"
# openssl returns an unsigned hexadecimal 4 bytes
hexNum=$(openssl rand -hex 4)
# printf $hexNum
# convert to unsigned decimal
unsigned=$((0x${hexNum}))
# printf $unsigned
# posgresql integer is 4 bytes signed - both plus and minus
# I want to have only positive numbers from 0 to 2147483647 (half of 4294967296)
if [ $unsigned -gt 2147483647 ]; then
    positiveDecNum=$((4294967296-$unsigned))
else
    positiveDecNum=$unsigned
fi
# printf $positiveDecNum

printf "\033[0;33m    Get random number from openssl: ${positiveDecNum} \033[0m\n"

printf "\033[0;33m    Use this number for the webpage_hit_counter badge:  \033[0m\n"
printf "\033[0;32m![$1](https://bestia.dev/webpage_hit_counter/get_svg_image/${positiveDecNum}.svg) \033[0m\n"

# bestia.dev ip without Cloudflare is 35.199.190.85
ssh luciano_bestia@35.199.190.85 \
" \
export PGPASSWORD=\"$password\"; \
psql -h localhost -p 5432 -U admin -d webpage_hit_counter -c \
    \" \
    insert into webpage (id, webpage) values($positiveDecNum, '$1'); \
    insert into hit_counter (webpage_id,count) \
    select id,2 \
    from webpage A \
    where A.id not in (select webpage_id from hit_counter); \
    \" \
"

