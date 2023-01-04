const int LED = 13;
const int SENSOR = 2;
int state = LOW;
int val = LOW;

/*
High level arduino code:

void setup() {
  pinMode(LED, OUTPUT);
  pinMode(SENSOR, INPUT);
}

void loop() {
  state = digitalRead(SENSOR);

  if (state == HIGH) {
    digitalWrite(LED, HIGH);

    delay(100);
  }

  if (state == LOW) {
    digitalWrite(LED, LOW);
  }
}
*/

uint8_t sensorCheck(void) {
  static uint8_t sensorState = 0;

  sensorState = (PINE & (1 << 4)) >> 4;

  if (sensorState == 1) {
    PORTB |= (1 << PB7);
  }

  if (sensorState == 0) {
    PORTB &= ~(1 << PB7);
  }
}

int main(void) {
  DDRE &= ~(1 << DDE4);
  DDRB |= (1 << DDB7);

  for (;;) {
    sensorCheck();
  }
}
