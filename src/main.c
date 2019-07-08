
// #define F_CPU 16000000

#include <avr/io.h>
// #include <util/delay.h>
#include "awo.h"

int main(void) {
    DDRB = 0xFF;
    for(;;) {
      do_flash();
    // _delay_ms(100);
    delay_test();
    PORTB = 0;
    delay_test();
    // _delay_ms(200);
    }
}