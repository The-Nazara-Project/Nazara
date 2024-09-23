#!/bin/bash

curl 'https://netbox-test.suse.de/api/dcim/devices/?name=Nazara-Test-Internal&name__empty=false&serial=PF23BEBC&serial__empty=false' \
  -H 'cookie: sessionid=kgpn9mre2fwsdlcowc6hq2anhih2ilfr'
