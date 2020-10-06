#include <stddef.h>
#ifdef __cplusplus
#define EXTERN extern "C"
#else
#define EXTERN extern
#endif
typedef void* (cdeno_op_callback)(void* interface, void* zero_copy_buf, size_t buf_len);

typedef struct {
    size_t len;
    void* data;
} ZeroCopyData;


EXTERN void cdeno_plugin_init(void* interface);
EXTERN size_t cdeno_register_op(void* interface, const char* name, cdeno_op_callback op_callback);
EXTERN void* cdeno_create_op_sync(unsigned char* data, size_t len);
EXTERN ZeroCopyData cdeno_get_zero_copy_buf(void* zero_copy_buf, size_t index);