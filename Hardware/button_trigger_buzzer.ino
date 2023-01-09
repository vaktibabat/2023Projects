#include <avr/io.h>
#include <util/delay.h>

enum t0_prescaler {
  T0_PRESCALER_1 = _BV(CS00),
  T0_PRESCALER_8 = _BV(CS01),
  T0_PRESCALER_64 = _BV(CS00) | _BV(CS01),
  T0_PRESCALER_256 = _BV(CS02),
  T0_PRESCALER_1024 = _BV(CS02) | _BV(CS00),
};

static void t0_set_prescaler(enum t0_prescaler ps) {
  TCCR0B = ps;
}

static unsigned short t0_get_prescaler_rate(enum t0_prescaler ps) {
  unsigned short rate;

  switch (ps) {
    case T0_PRESCALER_1:
      rate = 1;
      break;
    case T0_PRESCALER_8:
      rate = 8;
      break;
    case T0_PRESCALER_64:
      rate = 64;
      break;
    case T0_PRESCALER_256:
      rate = 256;
      break;
    case T0_PRESCALER_1024:
      rate = 1024;
      break;
  }
    
  return rate;
}

static unsigned long div_round(unsigned long d, unsigned long q) {
  return (d + (q / 2)) / q;
}

static t0_set_ctc_a(unsigned long hz, unsigned long timer_freq) {
  OCR0A = div_round(timer_freq, hz * 2) - 1;
  TCCR0A = _BV(COM0A0) | _BV(WGM01);
}

uint8_t buttonPressed(int timerFreq) {
  static uint8_t buttonState = 0;
  static uint8_t lastButtonState = 0;
  buttonState = (PINB & (1 << 5)) >> 5;

  if ((buttonState != lastButtonState) && (buttonState == 1)) {
    t0_set_ctc_a(5000, timerFreq);
    _delay_ms(200);
  }

  lastButtonState = buttonState;

  return buttonState;
}

int main(void) {
  DDRB |= (1 << DDB7);
  DDRB &= ~(1 << DDB5);
  PORTB &= ~(1 << PB5);
  PORTB &= ~(1 << PB7);

  enum t0_prescaler ps = T0_PRESCALER_256;
  unsigned long timerFreq;
  t0_set_prescaler(ps);
  timerFreq = div_round(F_CPU, t0_get_prescaler_rate(ps));
  
  for (;;) {
    ;
  }
}
