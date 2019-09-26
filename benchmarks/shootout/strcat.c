
#include <sightglass.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#define LENGTH 100000

#define STUFF "hello\n"

#ifdef WASM_ENTRY
#ifdef ALT_ENTRY
void
_call(int in)
#else
void
_start(void)
#endif
#else  // WASM_ENTRY
#ifdef EMBED_ENTRY
void
strcat_body(void *ctx_)
#else 
int
main(void)
#endif
#endif // WASM_ENTRY
{
    char * strbuf        = NULL;
    size_t sizeof_strbuf = (size_t) 1U;
    size_t stuff_len     = (sizeof STUFF) - (size_t) 1U;

    strbuf = malloc(LENGTH + sizeof_strbuf + (size_t) 1U);
#ifndef NO_WASI_SUPPORT
    assert(strbuf != NULL && LENGTH > stuff_len);
#endif
    strbuf[0] = 0;
    BLACK_BOX(stuff_len);
    while (sizeof_strbuf + stuff_len < LENGTH) {
        strcat(strbuf, STUFF);
        sizeof_strbuf += stuff_len;
    }

    strbuf[sizeof_strbuf - 1U] = 0;

    size_t res = strlen(strbuf);
    BLACK_BOX(res);

    free(strbuf);
}
