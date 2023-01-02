#include <avr/io.h>
#include <avr/interrupt.h>

volatile int isr_cnt = 0;

int main(void) {
  DDRB |= (1 << DDB5);
  TCCR0A |= (1 << WGM01);
  OCR0A = 124;
  TIMSK0 |= (1 << OCIE0A);
  sei();
  TCCR0B |= (1 << CS02);

  for (;;) {
    if (isr_cnt >= 250) {
      PORTB ^= (1 << PB5);
      isr_cnt = 0;
    }
  } 
}

ISR (TIMER0_COMPA_vect) {
  isr_cnt++;
}
