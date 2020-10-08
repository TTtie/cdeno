#include <stddef.h>
#ifdef __cplusplus
#define EXTERN extern "C"
#else
#define EXTERN extern
#endif
typedef void *(cdeno_op_callback)(void *interface, void *zero_copy_buf, size_t buf_len);
typedef void(cdeno_async_op_worker)(void *data, void *done_channel);

typedef struct
{
    size_t len;
    void *data;
} ZeroCopyData;

typedef struct
{
    void *data;
} CDenoAsyncOpData;

EXTERN void cdeno_plugin_init(void *interface);
EXTERN size_t cdeno_register_op(void *interface, const char *name, cdeno_op_callback op_callback);
EXTERN void *cdeno_create_op_sync(unsigned char *data, size_t len);
EXTERN ZeroCopyData cdeno_get_zero_copy_buf(void *zero_copy_buf, size_t index);
EXTERN void *cdeno_create_op_async(cdeno_async_op_worker worker, CDenoAsyncOpData data, bool unref = false);
EXTERN void cdeno_async_op_respond(void *done_channel, unsigned char *data, size_t len);