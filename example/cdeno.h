#include <stddef.h>
typedef void* (cdeno_op_callback)(void* interface, void* zero_copy_buf, size_t buf_len);

extern "C" void cdeno_plugin_init(void* interface);
extern "C" size_t cdeno_register_op(void* interface, const char* name, cdeno_op_callback op_callback);
extern "C" void* cdeno_create_op_sync(unsigned char* data, size_t len);
