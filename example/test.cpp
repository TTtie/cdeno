#include "cdeno.hpp"
#include <iostream>
#include <cstring>
#include <string>
#include <thread>
#include <chrono>

using namespace std;

void *test_op(void *interface, void *zero_copy_buf, size_t buf_len);
void test_op_async_worker(void *data, void *done_channel);
void *test_op_async(void *interface, void *zero_copy_buf, size_t buf_len);

extern "C" void cdeno_plugin_init(void *interface)
{
    cout << "Welcome to a cdeno plugin!!" << endl;
    cdeno_register_op(&interface, "test_op", &test_op);
    cdeno_register_op(&interface, "test_op_async", &test_op_async);
}

void *test_op(void *interface, void *zero_copy_buf, size_t buf_len)
{
    cout << "test_op called!" << endl;
    const char *test_response = "Hello world!";

    for (size_t i = 0; i < buf_len; i++)
    {
        ZeroCopyData copy_data = cdeno_get_zero_copy_buf(&zero_copy_buf, i);

        if (copy_data.len != 0)
        {
            string str((char *)copy_data.data, copy_data.len);
            cout << str << endl;
        }
    }

    void *op = cdeno_create_op_sync((unsigned char *)test_response, strlen(test_response));
    return op;
}
typedef struct
{
    const char *test_response;
} WorkerData;

void *test_op_async(void *interface, void *zero_copy_buf, size_t buf_len)
{
    cout << "test_op_async called!" << endl;
    const char *test_response = "Hello world!";

    cout << test_response << endl;
    for (size_t i = 0; i < buf_len; i++)
    {
        ZeroCopyData copy_data = cdeno_get_zero_copy_buf(&zero_copy_buf, i);

        if (copy_data.len != 0)
        {
            string str((char *)copy_data.data, copy_data.len);
            cout << str << endl;
        }
    }
    WorkerData *worker = (WorkerData *)malloc(sizeof(WorkerData));
    worker->test_response = test_response;
    CDenoAsyncOpData data = {worker};

    cout << "Worker data at " << &worker << endl;
    void *op = cdeno_create_op_async(&test_op_async_worker, data, true);
    return op;
}

void test_op_async_worker(void *data, void *done_channel)
{

    cout << "Hello from worker!" << endl;

    cout << "Data at " << data << " and done channel at " << done_channel << endl;

    // A thread is already spawned from Rust
    //thread thread([&done_channel, &data]() {
        WorkerData *worker_data = (WorkerData *)data;

        cout << "Sleeping from thread" << endl;
        this_thread::sleep_for(10000ms);
        cout << "Slept" << endl;
        cdeno_async_op_respond(done_channel,
                               (unsigned char *)(worker_data->test_response),
                               strlen(worker_data->test_response));
        free(worker_data);
    /*});

    thread.detach();*/
}