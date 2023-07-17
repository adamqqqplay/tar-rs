#!/bin/bash

SHELL_DIR=$(cd "$(dirname $0)";pwd)
cd "$SHELL_DIR" || exit

sudo rm -rf ./*.tar
echo "Hello, world!" >> "test.txt"
tar -cf "./smalluid_pax.tar" "./test.txt" --format=pax
sudo chown 4294967294:4294967294 "./test.txt"
tar -cf "./biguid_pax.tar" "./test.txt" --format=pax
tar -cf "./biguid_gnu.tar" "./test.txt" --format=gnu
sudo rm -rf "test.txt"