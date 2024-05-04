

```bash
clang -fPIC -c my_library.c -o my_library.o 
gcc -dynamiclib -o libmy_library.dylib my_library.o 
cp libmy_library.dylib /usr/local/lib 
```
# I think gcc is aliased to clang on Macos


