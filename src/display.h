#ifndef DISPLAY_H
#define DISPLAY_H

#include <stdbool.h>

//intialise display
void display_init();

//wait until the display is not busy
void display_wait_busy();

//send bits to display
void display_send_bits(bool rs_value, unsigned char data);

//send bits and wait until display is not busy
void display_send_bits_wait(bool rs_value, unsigned char data);

//initialise console. need to call display_init before this
void display_console_init(bool cursor_on, bool blink_on);

//write string. need display_console_init before this
void display_console_write(int col, int row, char* string);

#endif