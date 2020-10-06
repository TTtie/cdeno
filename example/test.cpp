#include "cdeno.h"
#include <iostream>

using namespace std;

void *test_op(void *interface, void *zero_copy_buf, size_t buf_len);
extern "C" void cdeno_plugin_init(void *interface)
{
    std::cout << "Welcome to a cdeno plugin!!" << std::endl;
    std::cout << "size of voidptr: " << sizeof(void *) << "" << sizeof(size_t) << std::endl;
    cdeno_register_op(interface, "test_op", test_op);
}

void *test_op(void *interface, void *zero_copy_buf, size_t buf_len)
{
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
        0x21};

    std::cout << (unsigned char *)&test_response << std::endl;
    std::cout << "Amount of zero-copy buffers: " << buf_len << std::endl;
    for (size_t i = 0; i < buf_len; i++)
    {
        ZeroCopyData copy_data = cdeno_get_zero_copy_buf(&zero_copy_buf, i);

        printf("Got zero copy data at index %d of length %d\n", i, copy_data.len);
        if (copy_data.len != 0)
        {
            fwrite(copy_data.data, sizeof(char), copy_data.len, stdout);
        }
    }

    void *op = cdeno_create_op_sync((unsigned char *)&test_response, sizeof(test_response));
    return op;
}