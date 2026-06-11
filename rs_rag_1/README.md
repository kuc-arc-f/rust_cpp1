# rs_rag_1

 Version: 0.9.1

 date    : 2026/06/10
 
 update :

***

Rust , C++ call , RAG SQLite

* embedding : Gemini-embedding-001
* model: gemma-4-E2B
* llama.cpp , llama-server
* LLVM CLang

***
* llama-server start
* port 8090: gemma-4-E2B

```
#gemma-4-E2B

/usr/local/llama-b8642/llama-server -m /var/lm_data/unsloth/gemma-4-E2B-it-Q4_K_S.gguf \
 --chat-template-kwargs '{"enable_thinking": false}' --port 8090 
```
***
### related

https://huggingface.co/unsloth/gemma-4-E2B-it-GGUF

***
* LIB add

```
sudo apt install uuid-dev
sudo apt install nlohmann-json3-dev
sudo apt install libsqlite3-dev
sudo apt install libcurl4-openssl-dev
```
***
* build

```
make all
cargo build
```

* start
```
export LD_LIBRARY_PATH=.
export GEMINI_API_KEY=
```
***
* use

* embed
```
./target/debug/rs_rag_1 embed
```

* search
```
./target/debug/rs_rag_1 search hello
```


***
### blog


***

