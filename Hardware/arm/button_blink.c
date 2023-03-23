int main(void)
{
  /* USER CODE BEGIN 1 */

  /* USER CODE END 1 */

  /* MCU Configuration--------------------------------------------------------*/

  /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
  HAL_Init();

  /* USER CODE BEGIN Init */

  /* USER CODE END Init */

  /* Configure the system clock */
  SystemClock_Config();

  /* USER CODE BEGIN SysInit */

  /* USER CODE END SysInit */

  /* Initialize all configured peripherals */
  MX_GPIO_Init();
  MX_I2C1_Init();
  MX_SPI1_Init();
  MX_USB_PCD_Init();
  MX_USART2_UART_Init();
  /* USER CODE BEGIN 2 */

  //HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_0);
  //Getting the MODER_A register according to the data sheet
  volatile uint32_t *MODER_A = (volatile uint32_t *)0x48000000;
  //Clearing bits 0, 1 which correspond to pin 0
  *MODER_A &= ~(0b11 << 0);
  //Writing the new value: 0b  00 which corresponds to the input mode
  *MODER_A |= (0b00 << 0);

  /* USER CODE END 2 */

  /* Infinite loop */
  /* USER CODE BEGIN WHILE */
  while (1)
  {
	  //Getting the IDR_A register according to the data sheet
	  volatile uint32_t *IDR_A = (volatile uint32_t *)(0x48000000 + 0x10);
	  uint32_t state = *IDR_A & (1 << 0);

	  //Checking if the button was pressed
	  if (state) {
		  HAL_Delay(10);
		  
		  HAL_GPIO_TogglePin(GPIOE, GPIO_PIN_10);
	  }
	  /* USER CODE END WHILE */

    /* USER CODE BEGIN 3 */
  }
  /* USER CODE END 3 */
}
