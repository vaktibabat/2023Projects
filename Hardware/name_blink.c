#include <avr/io.h>
#include <stdio.h>
#include <string.h>

#define FOSC 16000000
#define BAUD 9600
#define MYUBRR FOSC/16/BAUD-1

void USART_Init( unsigned int ubrr){
/* Set baud rate */
UBRR0H = (unsigned char)(ubrr>>8);
UBRR0L = (unsigned char)ubrr;
/* Enable receiver and transmitter */
UCSR0B = (1<<RXEN0)|(1<<TXEN0);
/* Set frame format: 8data, 2stop bit */
UCSR0C = (3<<UCSZ00);
} // USART_Init

void USART_Transmit( unsigned char data )
{
/* Wait for empty transmit buffer */
while ( ( UCSR0A & (1<<UDRE0)) == 0) {
  ;
}
/* Put data into buffer, sends the data */
UDR0 = data;
}

void USART_printf(char *s) {
  for (int i = 0; i < strlen(s); ++i) {
    USART_Transmit(s[i]);
  }
}

unsigned char USART_Receive( void )
{
/* Wait for data to be received */
while ( !(UCSR0A & (1<<RXC0)) ) {
  ;
}
/* Get and return received data from buffer */
return UDR0;
}

int main(void) {
    USART_Init(MYUBRR);
    
    DDRB |= (1 << DDB7);

    while (1) {
      USART_printf("What\'s your name? \n");
      unsigned char c;

      while ((c = USART_Receive()) != '#') {
        USART_Transmit(c);
        
        PORTB |= (1 << PB7);
        _delay_ms(500);
        PORTB &= ~(1 << PB7);
        _delay_ms(500);
      }

      USART_printf("Huh, that\'s a cool name!");
      USART_Transmit('\n');
    }

    return 0;
}
