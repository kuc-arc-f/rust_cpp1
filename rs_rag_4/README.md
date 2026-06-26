# rs_rag_4

 Version: 0.9.1

 date    : 2026/06/26
 
 update :

***

Rust , C++ call , RAG SQLite OpenRouter

* OpenRouter: openai/gpt-oss-20b:free
* embedding : Gemini-embedding-001
* LLVM CLang

***
### related

https://openrouter.ai/

https://openrouter.ai/models

***
* LIB add

```
sudo apt install uuid-dev
sudo apt install nlohmann-json3-dev
sudo apt install libsqlite3-dev
sudo apt install libcurl4-openssl-dev
```


***
### env value

```
export LD_LIBRARY_PATH=.
export GEMINI_API_KEY=your-key

export OPENROUTER_API_KEY=your-key
export OPENROUTER_MODEL=openai/gpt-oss-20b:free
```

***
* build

```
make all
cargo build
```

***
* use

```
sqlite3 ./example.db < table.sql
```
* embed
```
./target/debug/rs_rag_4 embed
```

* search
```
./target/debug/rs_rag_4 search hello
```


***
### blog


***

