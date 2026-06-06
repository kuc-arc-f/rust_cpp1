#pragma once
#include <cstdint>
#include <iostream>
#include <string>
#include <memory>
#include "rust/cxx.h"
#include "my_hoge.hpp"
#include "my_send.hpp"

std::int32_t add(
    std::int32_t a,
    std::int32_t b
) {
    return a + b;
}
std::unique_ptr<std::string> hoge_test() {
    MyHoge hoge("");
    std::string resp = hoge.test("hoge_test");
    return std::make_unique<std::string>(resp);
}
const std::string& hoge_test_2(const std::string& name) {
    MyHoge hoge("");
    static std::string resp = hoge.test_2(name);
    return resp;    
}

std::unique_ptr<std::string> send_test() {
    MySend send_helper("");
    send_helper.test();
    return std::make_unique<std::string>("OK");
}