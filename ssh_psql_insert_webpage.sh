#!/bin/sh

# ssh_psql_insert_webpage.sh

# README:

echo " "
echo "\033[0;33m    Bash script to insert a webpage in the webpage_hit_counter database on bestia.dev over SSH in one command. \033[0m"
echo "\033[0;33m    Use sshadd to enter the passcode for the SSH connection beforehand. \033[0m"
echo "\033[0;33m    The bash script needs 1 arguments: host_name like this: \033[0m"
echo "\033[0;33m    sh ssh_psql_insert_webpage.sh test.com \033[0m"
# repository: https://github.com/bestia-dev/webpage_hit_counter

if [ $# != 1 ]
then
  echo "Incorrect number of arguments"
  exit 1
fi

echo ""
# openssl returns an unsigned hexadecimal 4 bytes
hexNum=$(openssl rand -hex 4)
# echo $hexNum
# convert to unsigned decimal
unsigned=$((0x${hexNum}))
# echo $unsigned
# posgresql integer is 4 bytes signed - both plus and minus
# I want to have only positive numbers from 0 to 2147483647 (half of 4294967296)
if [ $unsigned -gt 2147483647 ]; then
    positiveDecNum=$((4294967296-$unsigned))
else
    positiveDecNum=$unsigned
fi
# echo $positiveDecNum

echo "\033[0;33m    Get random number from openssl: ${positiveDecNum} \033[0m"

echo "\033[0;33m    Use this number for the webpage_hit_counter badge:  \033[0m"
echo "\033[0;32m![$1](https://bestia.dev/webpage_hit_counter/get_svg_image/${positiveDecNum}.svg) \033[0m"

echo "\033[0;33m    You will be asked now for the password of the Postgres database.  \033[0m"

ssh luciano_bestia@bestia.dev \
" \
psql -h localhost -p 5432 -U admin -W -d webpage_hit_counter -c \
    \" \
    insert into webpage (id, webpage) values($positiveDecNum, '$1'); \
    insert into hit_counter (webpage_id,count) \
    select id,2 \
    from webpage A \
    where A.id not in (select webpage_id from hit_counter); \
    \" \
"

