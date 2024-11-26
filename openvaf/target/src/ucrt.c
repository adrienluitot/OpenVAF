#define NULL ((void *)0)
#define _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR 0x0002ULL

#ifdef __MINGW32__
#include <stddef.h>
#else
typedef void* _locale_t;
#endif

#ifdef __clang__
#include <stdio.h>  // Include for vsnprintf
#include <stdarg.h> // Include for va_list

// Clang-specific implementation of __stdio_common_vsprintf
int __stdio_common_vsprintf(unsigned __int64 options, char *str, size_t len, const char *format, _locale_t locale, va_list valist) {
    (void)options;  // Suppress unused variable warnings
    (void)locale;   // Suppress unused variable warnings
    return vsnprintf(str, len, format, valist); // Use standard library vsnprintf
}
#else
// GCC and other compilers
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

