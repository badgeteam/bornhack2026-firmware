#[macro_export]
#[rustfmt::skip]
macro_rules! board {
    // Power & Battery pins
    //($p:expr, charge)     => { $p.P0_05 }; // Low when charging
    //($p:expr, vbat)       => { $p.P0_31 }; // Analog input vbat
    //($p:expr, vbat_rd)    => { $p.P0_07 }; // Pull down to read vbat
    //($p:expr, ps_sync)    => { $p.P0_17 }; // Power mode buck/boost

    // RGB LED pins (low is on)
    ($p:expr, led_red)    => { $p.P1_07 };
    ($p:expr, led_green)  => { $p.P1_15 };
    ($p:expr, led_blue)   => { $p.P0_02 };

    // EPD display
    ($p:expr, epd_busy )  => { $p.P0_11 };
    ($p:expr, epd_reset)  => { $p.P0_12 };
    ($p:expr, epd_dc)     => { $p.P1_11 };
    ($p:expr, epd_csn)    => { $p.P1_09 };
    ($p:expr, epd_sck)    => { $p.P0_08 };
    ($p:expr, epd_data)   => { $p.P0_27 };
    ($p:expr, epd_spi)    => { $p.SPI3 };

    // Buzzer pin output (Square waveform generator)
    //($p:expr, buzzer)     => { $p.P0_13 };

    // Button Inputs (active low)
    ($p:expr, btn_can)    => { $p.P0_06 };
    ($p:expr, btn_exe)    => { $p.P0_26 };

    // Joystick pins (active low)
    ($p:expr, joy_up)     => { $p.P1_04 };
    ($p:expr, joy_down)   => { $p.P1_03 };
    ($p:expr, joy_left)   => { $p.P1_05 };
    ($p:expr, joy_right)  => { $p.P1_01 };
    ($p:expr, joy_fire)   => { $p.P1_02 };

    // LoRa SX1262 pins
    //($p:expr, lora_int)   => { $p.P0_29 }; // DIO1 from LoRa chip
    //($p:expr, lora_busy)  => { $p.P0_28 }; // Busy pin from LoRa chip
    //($p:expr, lora_rx_en) => { $p.P0_04 }; // Connected to RF switch, set high to enable receiving
    //($p:expr, lora_rst)   => { $p.P0_30 }; // Reset pin for LoRa chip (output)
    //($p:expr, lora_miso)  => { $p.P0_03 }; // SPI: MISO from LoRa chip
    //($p:expr, lora_mosi)  => { $p.P1_14 }; // SPI: MOSI to LoRa chip
    //($p:expr, lora_sck)   => { $p.P1_13 }; // SPI: SCK to LoRa chip
    //($p:expr, lora_nss)   => { $p.P1_12 }; // SPI: NSS to LoRa chip (active low chip select)
    //($p:expr, lora_spi)   => { $p.SPI2 };  // SPI peripheral for LoRa chip

    // I2C QWIIC pins
    //($p:expr, qwiic_sda)  => { $p.P1_10 };
    //($p:expr, qwiic_scl)  => { $p.P1_11 };

    // External Flash (QSPI flash)
    //($p:expr, flash_csn)  => { $p.P0_25 };
    //($p:expr, flash_sck)  => { $p.P0_21 };
    //($p:expr, flash_io0)  => { $p.P0_20 };
    //($p:expr, flash_io1)  => { $p.P0_24 };
    //($p:expr, flash_io2)  => { $p.P0_22 };
    //($p:expr, flash_io3)  => { $p.P0_23 };

    // GPIO pins
    //($p:expr, gpio_1 )    => { $p.P0_15 };
    //($p:expr, gpio_2 )    => { $p.P0_16 };
    //($p:expr, gpio_3 )    => { $p.P0_19 };
    //($p:expr, gpio_4 )    => { $p.P1_08 };
}
