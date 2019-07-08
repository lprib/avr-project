#define F_CPU 16000000
#include "pin.h"
#include "usart.h"
#include <stdbool.h>
#include <util/delay.h>

const pin_t RW = pd4;
const pin_t RS = pd3;
const pin_t EN = pd5;

const pin_t data_pins[] = { pd6, pd7, pb0, pb1, pb2, pb3, pb4, pb5 };

void
display_init()
{
  PIN_SET_OUTPUT(RS);
  PIN_SET_OUTPUT(RW);
  PIN_SET_OUTPUT(EN);

  for (int i = 0; i < 8; i++) {
    PIN_SET_OUTPUT(data_pins[i]);
  }

  // PIN_WRITE_LOW(RW);
}

void
display_wait_busy()
{
  // this pin becomes busy flag
  PIN_SET_INPUT(data_pins[7]);

  // set up instruction to query busy flag;
  PIN_WRITE_LOW(RS);
  PIN_WRITE_HIGH(RW);
  PIN_WRITE_HIGH(EN);

  bool isBusy = true;
  while (isBusy) {
    PIN_WRITE_HIGH(EN);
    isBusy = PIN_READ(data_pins[7]);
    PIN_WRITE_LOW(EN);
  }

  PIN_SET_OUTPUT(data_pins[7]);
}

void
display_send_bits(bool rs_value, unsigned char data)
{
  // set r/w to write
  //(assuming RW is always low for now)
  PIN_WRITE_LOW(RW);

  PIN_WRITE_VALUE(RS, rs_value);

  // write data to db pins
  for (int i = 0; i < 8; i++) {
    PIN_WRITE_VALUE(data_pins[i], data & (1 << i));
  }

  PIN_WRITE_HIGH(EN);
  // screen requires 160ns enable high time
  // use 3x nop to delay this amount assuming 16MHz clock on avr
  __asm__ volatile("nop\n\t"
                   "nop\n\t"
                   "nop\n\t");
  PIN_WRITE_LOW(EN);
}

void
display_send_bits_wait(bool rs_value, unsigned char data)
{
  display_send_bits(rs_value, data);
  display_wait_busy();
}

void
display_console_init(bool cursor_on, bool blink_on)
{
  // CLEAR
  display_send_bits_wait(0, 1);
  // HOME
  display_send_bits_wait(0, 0b10);
  // ENTRY MODE
  display_send_bits_wait(0, 0b1100 | (cursor_on << 1) | (blink_on));
}

void
display_console_write(int col, int row, char* string)
{
  int ddram_address = (row % 2) * 0x10 + (row >= 2) * 0x08 + col / 2;
  display_send_bits_wait(0, (1 << 7) | ddram_address);

  // if on an odd column, need to append a space since addresses only point to
  // even columns
  if (col % 2) {
    display_send_bits_wait(1, ' ');
  }

  while (*string != 0) {
    display_send_bits_wait(1, *string);
    string++;
  }
}