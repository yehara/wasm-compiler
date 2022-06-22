#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.wat
  actual=`wasmtime tmp.wat --invoke main`

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42

echo OK
