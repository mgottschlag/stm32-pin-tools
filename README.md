# stm32-pin-tools

This repository contains a collection of tools to aid with pin selection
during hardware and software development with STM32 microcontrollers.

- `stm32-pin-diagram` generates LaTeX figures showing the pins used by
  user-selected STM32 peripherals.

  Execute the program as follows to generate a pinout diagram for the `SPI2` and
  `I2C3` peripherals and for the STM32F429ZITx microcontroller:

      target/release/stm32-pin-diagram STM32F429ZITx -p SPI2 -p I2C3

  The microcontroller names correspond to the file names in the `cube-MX-db/mcu`
  directory, and the peripheral names correspond to the prefix of the pin names
  in those files or in the datasheet.

- `stm32-af-mapping` generates a textual reports of the pins providing alternate
  functions.

  Execute the program as follows to generate a report listing all alternate
  functions whose name contains "UART" supported by any microcontroller whose
  name contains "STM32F4". The report also shows which GPIO pins map to the
  alternate functions.

      target/release/stm32-af-mapping UART STM32F4

# Compiling

Install cargo as well as the rust compiler and then simply execute, for example,
`cargo build --release`.

# License

The code is licensed under the terms of the [MIT License](LICENSE).

