#ifndef DISPLAY_H
#define DISPLAY_H


#include <stdbool.h>
#include <avr/io.h>

//# python 

// for generating pin declarations and definitions
//# else generates h
//generate_c = True
//
//prefixes = ['b', 'c', 'd']
//
//if generate_c:
//for p in prefixes:
//for i in range(0, 8):
//print("pin_t p{0}{1} = {{&PORT{2}, &PIN{2}, &DDR{2}, {1}}};".format(p, i, p.upper()))
//print()
//else:
//#  generate h
//for p in prefixes:
//for i in range(0, 8):
//print("extern pin_t p{}{};".format(p, i));
//print()




typedef struct {
	volatile unsigned char* const portx;
	volatile unsigned char* const pinx;
	volatile unsigned char* const ddrx;
	unsigned char const pin_num;
} pin_t;

#define PIN_SET_INPUT(pin) ((*(pin).ddrx) &= ~(1<<(pin).pin_num))
#define PIN_SET_OUTPUT(pin)((*(pin).ddrx) |= (1<<(pin).pin_num))
#define PIN_WRITE_HIGH(pin) ((*(pin).portx) |= (1<<(pin).pin_num))
#define PIN_WRITE_LOW(pin) ((*(pin).portx) &= ~(1<<(pin).pin_num))
#define PIN_READ(pin) ((*(pin).pinx) &= (1<<(pin).pin_num))
#define PIN_WRITE_VALUE(pin, value) if(value) {PIN_WRITE_HIGH(pin);} else {PIN_WRITE_LOW(pin);}

// TODO rewrite with X Macros

#define pb0 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 0}
#define pb1 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 1}
#define pb2 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 2}
#define pb3 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 3}
#define pb4 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 4}
#define pb5 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 5}
#define pb6 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 6}
#define pb7 {.portx = &PORTB, .pinx = &PINB, .ddrx = &DDRB, .pin_num = 7}

#define pc0 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 0}
#define pc1 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 1}
#define pc2 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 2}
#define pc3 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 3}
#define pc4 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 4}
#define pc5 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 5}
#define pc6 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 6}
#define pc7 {.portx = &PORTC, .pinx = &PINC, .ddrx = &DDRC, .pin_num = 7}

#define pd0 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 0}
#define pd1 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 1}
#define pd2 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 2}
#define pd3 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 3}
#define pd4 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 4}
#define pd5 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 5}
#define pd6 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 6}
#define pd7 {.portx = &PORTD, .pinx = &PIND, .ddrx = &DDRD, .pin_num = 7}


#endif