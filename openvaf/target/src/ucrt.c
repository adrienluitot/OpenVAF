#define NULL ((void *)0)
#define _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR 0x0002ULL

#ifdef __MINGW32__
#include <stddef.h>
#else
typedef void* _locale_t;
#endif

#if defined(__clang__) && defined(_MSC_VER)
// For Clang targeting MSVC (Windows)
#include <stdarg.h>
#include <stdio.h>
int __cdecl __stdio_common_vsprintf(unsigned __int64 options, char *str, size_t len, const char *format, _locale_t locale, va_list valist) {
    (void)options;  // Suppress unused parameter warning
    (void)locale;   // Suppress unused parameter warning
    return vsnprintf(str, len, format, valist);  // Use standard `vsnprintf` for simplicity
}
#else
// For GCC and Clang on Linux
typedef char* va_list;
int __cdecl __stdio_common_vsprintf(unsigned __int64 options, char *str, size_t len, const char *format, _locale_t locale, va_list valist);
#endif

int __cdecl snprintf(char * __restrict__ __stream, size_t __n, const char * __restrict__ __format, ...) {
    __builtin_va_list ap;
    int ret;
    __builtin_va_start(ap, __format);
    ret = __stdio_common_vsprintf(_CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR, __stream, __n, __format, NULL, ap);
    __builtin_va_end(ap);
    return ret;
}

