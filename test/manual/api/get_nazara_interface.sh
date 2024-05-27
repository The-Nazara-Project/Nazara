#!/bin/bash

# Test NetBox's API response by manually requesting information.
# Must be run *after* running Nazara and registering your device.

# USAGE:
# ./test_interfaces.sh $URL $TOKEN

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <NetBox_URL> <Authorization_Token>"
  exit 1
fi

url=$1
token=$2


curl -X POST "$url/api/dcim/interfaces/" \
-H "Authorization: Token $token" \
-H "Content-Type: application/json" \
-d '{"name": "Nazara0", "device": 112, "type": "1000base-t"}' \
-o "./output/interfaces.json"

