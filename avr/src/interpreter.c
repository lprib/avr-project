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

      case OP_LOAD8:;
        stackval_t load_location =
          next_bytecode_val(program->bytecode, &instr_index);
        stack_push(
          program->stack, &stack_index, program->bytecode[load_location]);
        break;

      // note store8 only stores the LOWER 8 bits
      case OP_STORE8:
      case OP_STORE16:;
        stackval_t store_location =
          next_bytecode_val(program->bytecode, &instr_index);
        stackval_t value = stack_pop(program->stack, &stack_index);
        program->bytecode[store_location] = (unsigned char)(0xFF & value);
        // if 16 bits, store the top byte as well
        if (code == OP_STORE16) {
          program->bytecode[store_location + 1] = (unsigned char)(value >> 8);
        }
        break;

      case OP_JUMP:
        instr_index = next_bytecode_val(program->bytecode, &instr_index);
        break;

      // todo macro for all jump instrs
      case OP_JUMPNZ:
        if (stack_pop(program->stack, &stack_index) != 0) {
          instr_index = next_bytecode_val(program->bytecode, &instr_index);
        }
      break;
      
      case OP_JUMPZ:
        if(stack_pop(program->stack, &stack_index) == 0) {
          instr_index = next_bytecode_val(program->bytecode, &instr_index);
        }
      break;

      case OP_DROP:
        stack_index--;
        break;

      case OP_DUP:;
        stackval_t new = program->stack[stack_index];
        stack_push(program->stack, &stack_index, new);
        break;

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