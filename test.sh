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

assert 0 'main(){return 0;}'
assert 42 'main(){return 42;}'
assert 21 'main(){return 5+20-4;}'
assert 21 'main(){return  5 + 20 - 4 ;}'
assert 47 'main(){return 5+6*7;}'
assert 15 'main(){return 5*(9-6);}'
assert 4 'main(){return (3+5)/2;}'
assert 15 'main(){return -5*-3;}'
assert 5 'main(){return +30/+6;}'
assert 1 'main(){return 3*4==12;}'
assert 0 'main(){return 3*4==0;}'
assert 1 'main(){return 3*4!=0;}'
assert 0 'main(){return 3*4!=12;}'
assert 1 'main(){return 3*4>=12;}'
assert 0 'main(){return 3*4>=13;}'
assert 1 'main(){return 3*4<=12;}'
assert 0 'main(){return 3*4<=11;}'
assert 6 'main(){a=2;b=3;return a*b;}'
assert 6 'main(){aZ_1=2;BB=3;return aZ_1*BB;}'
assert 3 'main(){return 1+2; return 2*3;}'
assert 1 'main(){a=5;if(a>3)return 1;return 2;}'
assert 1 'main(){a=5;if(a>3){return 1;}{return 2;}}'
assert 2 'main(){a=3;if(a>3)return 1;return 2;}'
assert 1 'main(){a=5;if(a>3)return 1; else return 2;}'
assert 2 'main(){a=3;if(a>3)return 1; else return 2;}'
assert 1 'main(){a=5;if(a>3)b=1; else b=2;return b;}'
assert 2 'main(){a=3;if(a>3)b=1; else b=2;return b;}'
assert 5 'main(){a=1;while(a<=4)a=a+1;return a;}'
assert 15 'main(){a=0;for(i=1;i<=5;i=i+1)a=a+i;return a;}'
assert 20 'main(){a=0;for(i=1;i<=5;i=i+1){a=a+i;a=a+1;}return a;}'
assert 1 'main(){return sub();}sub(){return 1;}'
assert 3 'main(){return sub(5,2);}sub(a,b){return a-b;}'
assert 55 'main(){return fib1(10);}fib1(a){if(a<=1){return a;}return fib1(a-2)+fib1(a-1);}'
assert 55 'main(){return fib2(10);}fib2(a){if(a<=1){return a;}p0=0;p1=1;for(i=2;i<=a;i=i+1){p2=p0+p1;p0=p1;p1=p2;}return p2;}'
assert 60 'main(){return lcm(12,20);}lcm(a,b){return a/gcd(a,b)*b;}gcd(a,b){if(a<b)return gcd(b,a);if(a==b)return a;if(b==0)return a;return gcd(b,a-(a/b*b));}'
assert 2 'main(){return gcd(6,4);}lcm(a,b){return a/gcd(a,b)*b;}gcd(a,b){if(a<b)return gcd(b,a);if(a==b)return a;if(b==0)return a;return gcd(b, a-(a/b*b));}'
echo OK
