uint8_t buttonPressed(void) {
  static uint8_t buttonState = 0;
  static uint8_t lastButtonState = 0;

  buttonState = (PIND & (1 << 5)) >> 5;

  if ((buttonState != lastButtonState) && (buttonState == 1)) {
    PORTB ^= (1 << PB5);
  }

  lastButtonState = buttonState;

  return buttonState;
}

int main(void) {
  init();

  DDRB |= (1 << DDB5);
  PORTB |= (1 << PB5);
  DDRD &= ~(1 << DDD5);
  PORTD |= (1 << PD5);

  for (;;) {
    buttonPressed();
  }
}
