#ifndef USART_H
#define USART_H

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void
usart_init(uint32_t baudrate);

// waits until ready to write, and then writes data
void
usart_wait_write(unsigned char data);

bool
usart_read_ready();

bool
usart_write_ready();

// attempt to read without checking data availability
unsigned char
usart_read();

// wait until data is available and read it
unsigned char
usart_wait_read();

void
usart_write_bytes(unsigned char* data, size_t length);

// write string not including zero terminator
void
usart_write_string(char* string);

// write string and append newline
void
usart_write_line(char* string);

// writes data converted to a decimal string to usart, including a newline
void
usart_write_int_line(int data);

// same as usart_write_int_line but does so in hexadecimal
void
usart_write_int_hex_line(int data);

// reads usart until zero terminator is encountered (terminator is copied to
// buffer as well) NOTE: if the buffer fills before a zero is found, the string
// will not be terminated
size_t
usart_read_string(char* buffer, size_t buffer_size);

// reads until a newline is found
// appends a zero terminator after the newline
// if a newline is not found, buffer_size - 1 characters will be read, and a
// zero terminator will be added
size_t
usart_read_line(char* buffer, size_t buffer_size);

// read until either endbyte is found or buffer fills
size_t
usart_read_bytes_until(char* buffer, size_t buffer_size, char endbyte);

void
usart_flush_buffer();

#endif