#define NULL ((void *)0)
#define _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR 0x0002ULL

#ifdef __MINGW32__
#include <stddef.h>
#else
#ifndef _MSC_VER
typedef void* _locale_t;
#endif
#include <stdarg.h>
#endif

#ifdef _MSC_VER
int __cdecl __stdio_common_vsprintf(unsigned __int64 options, char *str, size_t len, const char *format, _locale_t locale, va_list valist);
#endif

int __cdecl snprintf (char * __restrict__ __stream, size_t __n, const char * __restrict__ __format, ...)
{
    __builtin_va_list ap;
    int ret;
    __builtin_va_start(ap, __format);
#ifdef _MSC_VER
    ret = __stdio_common_vsprintf(_CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR, __stream, __n, __format, NULL, ap);
#else
    ret = vsnprintf(__stream, __n, __format, ap);
#endif
    __builtin_va_end(ap);
    return ret;
}

