#include <sightglass.h>

#include <ctype.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#define ITERATIONS 1000000

#define MINICSV_DELIM ','
#define MINICSV_QUOTE '"'

static inline void
_minicsv_out_col(char* const col_start, char** const cols, size_t* const cols_count_p,
    const size_t cols_max)
{
    if (*cols_count_p < cols_max) {
        cols[*cols_count_p] = col_start;
    }
    (*cols_count_p)++;
}

static inline void
_minicsv_parse_line_noquote(const int c, char** const pos_p, char** const col_start_p,
    int* const state_p, char** const cols, size_t* const cols_count_p,
    const size_t cols_max)
{
    switch (c) {
    case '\n':
        *state_p = 2;
    case MINICSV_DELIM:
        *(*pos_p)++ = 0;
        _minicsv_out_col(*col_start_p, cols, cols_count_p, cols_max);
        *col_start_p = *pos_p;
        break;
    case MINICSV_QUOTE:
        *state_p = 1;
        *col_start_p = ++ * pos_p;
        break;
    case '\r':
        **pos_p = 0;
    default:
        (*pos_p)++;
    }
}

static inline void
_minicsv_parse_line_quote(const int c, char** const pos_p, int* const state_p)
{
    if (c == MINICSV_QUOTE) {
        if (*(*pos_p + 1U) == MINICSV_QUOTE) {
            memmove(*pos_p, *pos_p + 1U, strlen(*pos_p));
        }
        else {
            **pos_p = 0;
            *state_p = 0;
        }
    }
    (*pos_p)++;
}

static char*
minicsv_parse_line(char* const buf, char** const cols, size_t* const cols_count_p,
    const size_t cols_max)
{
    char* col_start = buf;
    char* pos = buf;
    int   c;
    int   state = 0;

    *cols_count_p = (size_t)0U;
    while ((c = *pos) != 0) {
        switch (state) {
        case 0:
            _minicsv_parse_line_noquote(c, &pos, &col_start, &state, cols, cols_count_p, cols_max);
            break;
        case 1:
            _minicsv_parse_line_quote(c, &pos, &state);
            break;
        case 2:
            return pos;
        }
    }
    if (state == 0) {
        _minicsv_out_col(col_start, cols, cols_count_p, cols_max);
    }
    return pos;
}

static void
minicsv_trim_cols(char** const cols, const size_t cols_count)
{
    char* col;
    size_t col_end;
    size_t i = (size_t)0U;

    while (i < cols_count) {
        col = cols[i];
        while (*col != 0 && isspace((int)(unsigned char)*col)) {
            cols[i] = ++col;
        }
        col_end = strlen(col);
        while (col_end > (size_t) 0U && isspace((int)(unsigned char)col[--col_end])) {
            col[col_end] = 0;
        }
        i++;
    }
}

int main()
{
    char* cols[10];
    size_t cols_count;
    int    i;

    bench_start();
    for (i = 0; i < ITERATIONS; i++) {
        char buf[] =
            "first,line,has,\"comas,\"\"escaped\"\" characters\",and,\"multiples\r\nlines\"\r\n"
            "second,line,\"  has  \",,empty,,,,columns\r\nremainder";
        char* r = buf;
        BLACK_BOX(buf);
        r = minicsv_parse_line(r, cols, &cols_count, sizeof cols / sizeof cols[0]);
        if (cols_count > sizeof cols / sizeof cols[0]) {
            abort();
        }
        minicsv_trim_cols(cols, cols_count);
        BLACK_BOX(cols);
        r = minicsv_parse_line(r, cols, &cols_count, sizeof cols / sizeof cols[0]);
        if (cols_count > sizeof cols / sizeof cols[0]) {
            abort();
        }
        minicsv_trim_cols(cols, cols_count);
        BLACK_BOX(cols_count);
        BLACK_BOX(r);
    }
    bench_end();
}
