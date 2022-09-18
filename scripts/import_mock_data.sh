#!/bin/bash
set -uex

function import_test_data() {
  for f in db/tests/*.sql; do
    psql postgres://scroll:scroll2022@localhost:5434/scroll -f $f
  done
}

import_test_data
