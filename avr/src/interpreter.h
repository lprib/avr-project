#ifndef INTERPRETER_H
#define INTERPRETER_H

#include <stddef.h>

typedef unsigned int stackval_t;

typedef struct {
    unsigned char* bytecode;
    stackval_t* stack;
    size_t bytecode_size;
    size_t stack_size;
} program_t;

void
run_program(program_t* program);

stackval_t
next_bytecode_val(unsigned char* bytecode, unsigned int* instr_index);

stackval_t
stack_pop(stackval_t* stack, unsigned int* stack_index);

void
stack_push(stackval_t* stack, unsigned int* stack_index, stackval_t value);

#endif