cargo build --release
g++ -O3 -march=native -fPIC -shared ./test.cpp -L$PWD/../target/release -lcdeno -o cdeno-test-lib.so
