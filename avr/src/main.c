#define F_CPU 16000000

#include "interpreter.h"
#include "usart.h"
#include <stdbool.h>
#include <util/delay.h>

int
main(void)
{
  usart_init(9600);

  unsigned char bytecode[] = { 0x00, 0x01, 0x8F, 0x00, 0x00,
                               0x15, 0x20, 0x50, 0x00, 0x00 };
  stackval_t stack[300] = { 0 };

  program_t program = { bytecode,
                        stack,
                        sizeof(bytecode) / sizeof(unsigned char),
                        sizeof(stack) / sizeof(stackval_t) };

  run_program(&program);

  for (;;) {
  }
}