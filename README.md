# stm32-pin-diagram

This program generates LaTeX figures showing the pins used by different STM32
peripherals.

# Compiling

Install cargo as well as the rust compiler and then simply execute, for example,
`cargo build --release`.

# Usage

Execute the program as follows to generate a pinout diagram for the `SPI2` and
`I2C3` peripherals and for the STM32F429ZITx microcontroller:

    target/release/stm32-pin-diagram STM32F429ZITx -p SPI2 -p I2C3

The microcontroller names correspond to the file names in the `cube-MX-db/mcu`
directory, and the peripheral names correspond to the prefix of the pin names in
those files or in the datasheet.

