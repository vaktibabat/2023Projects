#include <avr/io.h>

int main(void) {
	DDRB |= (1 << DDB7);
	DDRE &= ~(1 << DDE4);

	PORTE |= (1 << PE4);

	for (;;) {
		int sensorState;

		sensorState = (PINE & (1 << PE4)) >> PE4;
		
		if (sensorState) {
			PORTB |= (1 << PB7);
		}
		else {
			PORTB &= ~(1 << PB7);
		}
	}
}
	
