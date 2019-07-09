#ifndef LBC_SYSCALLS_H
#define LBC_SYSCALLS_H

#include "lbc_runner.h"

// syscall function pointer
typedef void (*syscall_function_t)(ExecutionContext*);

extern const syscall_function_t syscalls[];

#endif