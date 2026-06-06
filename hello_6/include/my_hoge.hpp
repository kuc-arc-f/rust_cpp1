#pragma once
#include <iostream>
#include "rust/cxx.h"

class MyHoge {
private:
    std::string m_name;

    public:
    explicit MyHoge(std::string str){}

    ~MyHoge() {}

    std::string test(std::string str) {
      std::cout << str << " \n";
      return str;
    }

    std::string test_2(std::string input) {
      std::string buff = "HelloTest2-" + input + "!";
      return buff;
    }

};

