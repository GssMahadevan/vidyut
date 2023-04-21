#!/bin/bash

find . -name '*.rs'|xargs perl -ne 'if (/(\d{1})\.(\d{1})\.(\d+)/) { print "$1\.$2.$3\n"; }'  |sort|uniq -c