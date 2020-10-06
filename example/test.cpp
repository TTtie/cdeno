#include "cdeno.h"
#include <iostream>

using namespace std;

cdeno_op_callback test_op;

void* test_op(void* interface, void* zero_copy_buf, size_t buf_len); 
extern "C" void cdeno_plugin_init(void* interface) {
    std::cout << "Welcome to a cdeno plugin!!" << std::endl;
    std::cout << "size of voidptr: " << sizeof(void*) << "" << sizeof(size_t) << std::endl;
    cdeno_register_op(interface, "test_op", &test_op);
}

void* test_op(void* interface, void* zero_copy_buf, size_t buf_len) {
    std::cout << "test_op called!" << std::endl;
    const char test_response[] = {
        0x48,
        0x65,
        0x6c,
        0x6c,
        0x6f,
        0x20,
        0x77,
        0x6f,
        0x72,
        0x6c,
        0x64,
        0x21
    };
    
    std::cout << (unsigned char *) &test_response << std::endl;
    void* op = cdeno_create_op_sync((unsigned char *) &test_response, sizeof(test_response));
    return op;
}