#ifndef SYSCALLS_H
#define SYSCALLS_H

#include "interpreter.h"

typedef void (*syscall_function_t)(program_t* program, unsigned int* stack_index);

extern const syscall_function_t syscalls[];

#endif