#include "interpreter.h"
#include "opcodes.h"
#include "syscalls.h"
#include <stdbool.h>
#include <stddef.h>

#define BINARY_OP_CASE(opcode, op)                                             \
  case opcode:                                                                 \
    stack_index -= 1;                                                          \
    stackval_t opcode##_val =                                                  \
      program->stack[stack_index] op program->stack[stack_index + 1];          \
    program->stack[stack_index] = opcode##_val;                                \
    break

void
run_program(program_t* program)
{
  unsigned int stack_index = 0;
  unsigned int instr_index = 0;
  unsigned int locals_index = 0;

  while (true) {
    unsigned char code = program->bytecode[instr_index];
    instr_index++;

    switch (code) {
      case OP_PUSH:;
        stackval_t val = next_bytecode_val(program->bytecode, &instr_index);
        stack_push(program->stack, &stack_index, val);
        break;
      case OP_SYSCALL:;
        stackval_t syscall_num =
          next_bytecode_val(program->bytecode, &instr_index);
        syscalls[syscall_num](program, &stack_index);
        break;

        BINARY_OP_CASE(OP_ADD, +);
        BINARY_OP_CASE(OP_SUB, -);
        BINARY_OP_CASE(OP_MUL, *);
        BINARY_OP_CASE(OP_DIV, /);
        BINARY_OP_CASE(OP_MOD, %);
      default:
        break;
    }

    if (instr_index >= program->bytecode_size) {
      break;
    }
  }
}

stackval_t
next_bytecode_val(unsigned char* bytecode, unsigned int* instr_index)
{
  stackval_t ret = (bytecode[*instr_index] << 8) + (bytecode[*instr_index + 1]);
  (*instr_index) += 2;
  return ret;
}

stackval_t
stack_pop(stackval_t* stack, unsigned int* stack_index)
{
  (*stack_index)--;
  return stack[(*stack_index) + 1];
}

void
stack_push(stackval_t* stack, unsigned int* stack_index, stackval_t value)
{
  (*stack_index)++;
  stack[*stack_index] = value;
}