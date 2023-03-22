

run: build
	./c_stuff

build: c_stuff.o
	gcc -L ../target/debug -l ffi_stuff c_stuff.o -o c_stuff

c_stuff.o: main.c c_stuff.h
	gcc -c main.c -o c_stuff.o

clean:
	rm -rf *.o c_stuff main
