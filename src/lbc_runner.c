#include "lbc_runner.h"
#include "lbc_opcodes.h"
#include "lbc_syscalls.h"

#include "usart.h"

#define NUM_FUNCTIONS_OFFSET 0
#define FUNCTION_TABLE_OFFSET 2

// size in bytes of each entry to the function table
#define FUNCTION_TABLE_ENTRY_SIZE 2

// the number of bytes at the start of a function in bytecode that are not
// actual code only 2 at this point in time to define local allocation size
#define FUNCTION_HEADER_SIZE 2

#define BINARY_OP_CASE(opcode, op)                                             \
  case opcode:                                                                 \
    context.stack_ptr -= 1;                                                    \
    stackval_t opcode##_val = context.stack_ptr[0] op context.stack_ptr[1];    \
    context.stack_ptr[0] = opcode##_val;                                       \
    break

unsigned int
get_int_at_index(unsigned char* byte_array, int index);

stackval_t
next_bytecode_int(ExecutionContext* context, int* instruction_pointer);

stackval_t
stack_pop(ExecutionContext* context);

void
stack_push(ExecutionContext* context, stackval_t value);

// takes context by value because we want to edit it (change stack ptr, etc)
// without changing the values in the parent function
void
execute_function(ExecutionContext context, int function_index)
{
  // the index in the bytecode of the function table entry
  int function_table_entry_index =
    FUNCTION_TABLE_OFFSET + FUNCTION_TABLE_ENTRY_SIZE * function_index;
  unsigned int function_start =
    get_int_at_index(context.bytecode, function_table_entry_index);

  unsigned int locals_allocation_size =
    get_int_at_index(context.bytecode, function_start);
  // since the context is passed by copy (not reference),
  // we can increment the pointer without affecting parent functions
  context.stack_ptr += locals_allocation_size;

  // add 2 to skip the locals allocation declaration
  int function_base_code_ptr = function_start + 2;

  int instruction_pointer = function_base_code_ptr;

  while (1) {

    switch (context.bytecode[instruction_pointer++]) {
      case OP_DEBUGPRINTCONST:
        usart_write_int_line(next_bytecode_int(&context, &instruction_pointer));
        break;

      case OP_DEBUGPRINTSTACK:
        usart_write_int_line(stack_pop(&context));
        break;

      case OP_RETURN:
        // todo cleanup etc.
        return;
        break;

      case OP_PUSHCONST:
        stack_push(&context, next_bytecode_int(&context, &instruction_pointer));
        // stack math:
        break;

      case OP_SYSCALL:
        (*syscalls[next_bytecode_int(&context, &instruction_pointer)])(
          &context);
        break;

        BINARY_OP_CASE(OP_ADD, +);
        BINARY_OP_CASE(OP_SUB, -);
        BINARY_OP_CASE(OP_MUL, *);
        BINARY_OP_CASE(OP_DIV, /);
        BINARY_OP_CASE(OP_MOD, %);

      default:
        break;
    }
  }
}

// big endian
unsigned int
get_int_at_index(unsigned char* byte_array, int index)
{
  return (unsigned int)(byte_array[index] << 8) | byte_array[index + 1];
}

// consumes an int from the instruction stream, modifying instruction_pointer
// accordingly offset is the amount of bytes after the current instruction
// pointer
stackval_t
next_bytecode_int(ExecutionContext* context, int* instruction_pointer)
{
  stackval_t ret = (context->bytecode[*instruction_pointer] << 8) +
                   context->bytecode[*instruction_pointer + 1];
  *instruction_pointer += 2;
  return ret;
}

// pass execution context by ref because we want the changes
// to be reflected in the function that calls this
stackval_t
stack_pop(ExecutionContext* context)
{
  context->stack_ptr--;
  return context->stack_ptr[1];
}

void
stack_push(ExecutionContext* context, stackval_t value)
{
  context->stack_ptr++;
  context->stack_ptr[0] = value;
}