#define F_CPU 16000000
#include "awo.h"
#include <avr/io.h>
#include <util/delay.h>


void do_flash() {
    PORTB = 0xff;
}

void delay_test() {
    _delay_ms(400);
}