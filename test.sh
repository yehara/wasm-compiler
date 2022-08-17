#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run -- "$input" > tmp.wat
  actual=`wasmtime tmp.wat --invoke main`

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 '0;'
assert 42 '42;'
assert 21 '5+20-4;'
assert 21 ' 5 + 20 - 4 ;'
assert 47 '5+6*7;'
assert 15 '5*(9-6);'
assert 4 '(3+5)/2;'
assert 15 '-5*-3;'
assert 5 '+30/+6;'
assert 1 '3*4==12;'
assert 0 '3*4==0;'
assert 1 '3*4!=0;'
assert 0 '3*4!=12;'
assert 1 '3*4>=12;'
assert 0 '3*4>=13;'
assert 1 '3*4<=12;'
assert 0 '3*4<=11;'
assert 6 'a=2;b=3;a*b;'
assert 6 'aZ_1=2;BB=3;aZ_1*BB;'
assert 3 'return 1+2; return 2*3;'
assert 1 'a=5;if(a>3)return 1;return 2;'
assert 2 'a=3;if(a>3)return 1;return 2;'
assert 1 'a=5;if(a>3)return 1; else return 2;'
assert 2 'a=3;if(a>3)return 1; else return 2;'
assert 1 'a=5;if(a>3)b=1; else b=2;return b;'
assert 2 'a=3;if(a>3)b=1; else b=2;return b;'
echo OK
