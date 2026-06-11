#include <iostream>
#include <string>
#include <cstring>
#include "include/my_rag.hpp"

extern "C" {

    int add(int a, int b)
    {
        return a + b;
    }

    // メモリ解放用関数
    void free_string(char* ptr)
    {
        delete[] ptr;
    }

    char* rag_add() {
        MyRag todo_helper("");
        todo_helper.rag_add_handler();
        std::string result = "OK";
        char* output = new char[result.length() + 1];
        strcpy(output, result.c_str());
        return output;    
    }  

    char* rag_search(const char* input) {
        std::string input_str(input);
        //std::cout << "todo_add.Received in C++: " << input_str << std::endl;
        MyRag todo_helper("");
        std::string result = todo_helper.rag_search_handler(input_str);
        char* output = new char[result.length() + 1];
        strcpy(output, result.c_str());
        return output;    
    }    

}