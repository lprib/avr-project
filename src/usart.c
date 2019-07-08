#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <avr/io.h>
#include <stdio.h>

#include "util.h"
#include "usart.h"

#ifndef F_CPU
#define F_CPU 16000000UL
#endif

void set_baudrate(uint32_t baudrate);

void usart_init(uint32_t baudrate) {
	//todo make a version of this with all parameters (frame size, parity, etc)
	
	UCSR0A = 0x00;
	//enable receive and transmit
	UCSR0B = BIT(RXEN0) | BIT(TXEN0);
	//set frame size to 8 bit
	UCSR0C = BIT(UCSZ00) | BIT(UCSZ01);
	
	set_baudrate(baudrate);
}

void usart_wait_write(unsigned char data) {
	//wait for transmit buffer to be empty
	while(!usart_write_ready());
	
	UCSR0A |= BIT(TXC0);
	
	UDR0 = data;
}

bool usart_read_ready() {
	return UCSR0A & BIT(RXC0);
}

bool usart_write_ready() {
	return UCSR0A & BIT(UDRE0);
}

unsigned char usart_read() {
	return UDR0;
}

unsigned char usart_wait_read() {
	while(!usart_read_ready());
	return usart_read();
}

void usart_write_bytes(unsigned char* data, size_t length) {
	for(int i = 0; i < length; i++)
	{
		usart_wait_write(data[i]);
	}
}

void usart_write_string(char* string) {
	while(*string != 0)
	{
		usart_wait_write(*(string++));
	}
}

void usart_write_line(char* string) {
	usart_write_string(string);
	usart_wait_write('\n');
}

void usart_write_int_line(int data) {
	char string[15];
	sprintf(string, "%d", data);
	usart_write_line(string);
}

void usart_write_int_hex_line(int data) {
	char string[6];
	sprintf(string, "%X", data);
	usart_write_line(string);
}

size_t usart_read_string(char* buffer, size_t buffer_size) {
	return usart_read_bytes_until(buffer, buffer_size, 0);
}

size_t usart_read_line(char* buffer, size_t buffer_size) {
	//note the buffer size is one less when passed on to accommodate zero terminator
	size_t num_bytes_read = usart_read_bytes_until(buffer, buffer_size - 1, '\n');
	//add zero terminator
	buffer[num_bytes_read] = 0;
	return num_bytes_read + 1;
}

size_t usart_read_bytes_until(char* buffer, size_t buffer_size, char endbyte) {
	int i;
	for(i = 0; i < buffer_size; i++)
	{
		char c = usart_wait_read();
		buffer[i] = c;
		
		if(c == endbyte)
		{
			//successfully read full string
			break;
		}
	}
	
	//reached full buffer without finding endbyte
	return i + 1;
}

#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
void usart_flush_buffer() {
	unsigned char dummy;
	while(UCSR0A & BIT(RXC0))
	{
		//read values into dummy var to remove them from the buffer
		dummy = UDR0;
	}
}

void set_baudrate(uint32_t baudrate) {
	uint32_t ubrr = (F_CPU / (16*baudrate)) - 1;
	UBRR0H = ubrr >> 8;
	UBRR0L = ubrr & 0xFF;
}