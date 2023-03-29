#!/bin/bash

# Exit if not in git root
if [ ! -d .git ]; then
    echo "Not in git root"
    exit 1
fi

rm -rf tests/output tests/working_test_data
mkdir tests/working_test_data
cp -vp tests/real_videos/* tests/working_test_data/
