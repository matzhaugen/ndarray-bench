PHONY=.bench
cpp:
	g++ -std=c++11 -I eigen-3.3.7 -o main.o src/cpp/main.cpp
rs:
	cargo build --release
bench:
	./main.o
	cargo run
