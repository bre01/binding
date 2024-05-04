


```bash
 gcc -fPIC -c my_library.c -o my_library.o
gcc -shared -o libmy_library.so my_library.o
cp ~/source/binding/rust/scalar_linux/src/libmy_library.so  /usr/local/lib 
cargo run
```

I don't know why LD_LIBRARY_PATH environment variable does not work


