#!/usr/bin/env bash

. .env

echo $EMAIL
echo $ZONE
echo $API_TOKEN

COMMAND=`curl https://api.cloudflare.com/client/v4/zones/$ZONE/dns_records \
    -H "X-Auth-Email: $EMAIL" \
    -H "X-Auth-Key: $API_KEY"`

echo $COMMAND > output.json