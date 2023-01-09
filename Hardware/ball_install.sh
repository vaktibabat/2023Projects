avr-gcc -Os -DF_CPU=16000000L -mmcu=atmega2560 -c -o ball.o ball.c
avr-gcc -mmcu=atmega2560 ball.o -o ball
avr-objcopy -O ihex -R .eeprom ball ball.hex
avrdude -F -c stk500v2 -p m2560 -P /dev/ttyACM0 -b 115200 -D -U flash:w:ball.hex
