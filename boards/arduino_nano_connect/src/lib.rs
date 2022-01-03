#![no_std]

pub extern crate rp2040_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

//// The linker will place this boot block at the start of our program image. We
//// need this to help the ROM bootloader get our code up and running.
#[cfg(feature = "boot2")]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

use display_interface_spi::SPIInterface;
use embedded_hal::{
    adc::{Channel, OneShot},
    digital::v2::{InputPin, OutputPin},
    spi::MODE_0,
};

use embedded_time::rate::*;

pub use hal::pac;

use hal::{
    adc::Adc,
    gpio::{
        bank0::{
            Gpio0, Gpio1, Gpio12, Gpio13, Gpio14, Gpio15, Gpio16, Gpio17, Gpio18, Gpio19, Gpio2,
            Gpio20, Gpio21, Gpio22, Gpio23, Gpio24, Gpio25, Gpio26, Gpio27, Gpio28, Gpio29, Gpio3,
            Gpio4, Gpio5, Gpio6, Gpio7,
        },
        FunctionI2C, FunctionPwm, FunctionSpi, Pin, PinId, PullUpInput, PushPullOutput,
    },
    pac::{RESETS, SPI0},
    sio::SioGpioBank0,
    spi::{Enabled, Spi},
};

hal::bsp_pins!(
    Gpio0 { name: tx },
    Gpio1 { name: rx },
    Gpio2 { name: gpio0 },
    Gpio3 { name: reset },
    Gpio4 { 
        name: miso,
        aliases: { FunctionSpi: Miso }
    },
    Gpio5 { name: d10 },
    Gpio6 { 
        name: sck,
        aliases: { FunctionSpi: Sclk }
    },
    Gpio7 { 
        name: spitx, 
        aliases: { FunctionSpi: Spitx }
    },
    Gpio8 { name: gpio8 },
    Gpio9 { name: gpio9 },
    Gpio10 { name: gpio10 },
    Gpio11 { name: gpio11 },
    Gpio12 { name: gpio12 },
    Gpio13 { name: gpio13 },
    Gpio14 { name: gpio14 },
    Gpio15 { name: gpio15 },
    Gpio16 { name: gpio16 },
    Gpio17 { name: gpio17 },
    Gpio18 { name: gpio18 },
    Gpio19 { name: gpio19 },
    Gpio20 { name: gpio20 },
    Gpio21 { name: gpio21 },
    Gpio22 { name: gpio22 },
    Gpio23 { name: b_power_save },
    Gpio24 { name: vbus_detect },
    Gpio25 { name: led },
    Gpio26 { name: gpio26 },
    Gpio27 { name: gpio27 },
    Gpio28 { name: gpio28 },
    Gpio29 {
        name: voltage_monitor
    },
);

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
