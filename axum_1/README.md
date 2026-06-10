# axum_1

 Version: 0.9.1

 date    : 2026/06/08
 
 update :

***

Rust , C++ call , Axum server

* SQLite database
* rustc 1.94.0 
* LLVM Clang
* Linux

***
* LIB add
```
sudo apt-get install libsqlite3-dev
sudo apt install nlohmann-json3-dev
```
***
* build

```
make all
cargo build
```

* start
* localhost:3000 start

```
export LD_LIBRARY_PATH=.
./target/debug/api_server
```

***
* test-code

* add
```
curl -X POST http://localhost:3000/api/todo_add \
  -H "Content-Type: application/json" \
  -d '{
    "input": "test1"
  }'
```

* delete
```
curl -X POST http://localhost:3000/api/todo_delete \
  -H "Content-Type: application/json" \
  -d '{
    "id": 2
  }'
```



***
### blog

https://zenn.dev/knaka0209/scraps/3c9f5fbf87f96c

***

