#include "syscalls.h"
#include "interpreter.h"
#include "usart.h"

void syscall_usart_write(program_t* program, unsigned int* stack_index) {
    usart_write_int_line(stack_pop(program->stack, stack_index));
}

const syscall_function_t syscalls[] = {
    &syscall_usart_write
};