#pragma once
#include <iostream>
#include <string>
#include "rust/cxx.h"
#include "http_client.hpp"

class MySend {
private:
    std::string m_name;
public:
    explicit MySend(std::string str){}
    ~MySend() {}

    
    // ─────────────────────────────────────────────
    // ユーティリティ：レスポンス表示
    // ─────────────────────────────────────────────
    static void print_response(const std::string& label, const HttpResponse& resp)
    {
        std::cout << "\n===== " << label << " =====\n";
        if (!resp.error.empty()) {
            std::cerr << "[ERROR] " << resp.error << "\n";
            return;
        }
        std::cout << "Status : " << resp.status_code << "\n";
        std::cout << "Body   :\n" << resp.body << "\n";
    }

    void test() {
        std::string url = "http://localhost";
        HttpClient client(30 /*timeout*/, true /*verify_ssl*/);
        auto resp = client.get(url);
        print_response("GET:", resp);
    }
};


