#ifndef LBC_RUNNER_H
#define LBC_RUNNER_H

typedef unsigned int stackval_t;

typedef struct
{
  unsigned char* bytecode;
  stackval_t* stack_bottom;
  stackval_t* stack_ptr;
} ExecutionContext;

void
execute_function(ExecutionContext context, int function_index);

stackval_t
next_bytecode_int(ExecutionContext* context, int* instruction_pointer);

stackval_t
stack_pop(ExecutionContext* context);

void
stack_push(ExecutionContext* context, stackval_t value);

#endif