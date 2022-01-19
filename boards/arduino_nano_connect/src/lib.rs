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
            Gpio0, Gpio1, Gpio25, Gpio15, Gpio16, Gpio17, Gpio18, Gpio19, Gpio20,
            Gpio21, Gpio5, Gpio7, Gpio4, Gpio6, Gpio26, Gpio27, Gpio28, Gpio29,
            Gpio12, Gpio13,
        },
        FunctionI2C, FunctionPwm, FunctionSpi,
        Pin, PinId, PullUpInput, PushPullOutput,
    },
    pac::{RESETS, SPI0},
    sio::SioGpioBank0,
    spi::{Enabled, Spi},
};

mod internal_pins {
    hal::bsp_pins!(
        Gpio0   { name: tx,
                  aliases: { name: gpio0 }  },
        Gpio1   { name: rx,
                  aliases: { name: gpio1 }  },
        Gpio2   { name: gpio0 },
        Gpio3   { name: rstn_nina },
        Gpio4   { name: cipo,
                  aliases: { name: d12, FunctionSpi: Miso }     },
        Gpio5   { name: d10,
                  aliases: { name: gpio5 }  },
        Gpio6   { name: sck,
                  aliases: { name: d13, FunctionSpi: SpiClk }   },
        Gpio7   { name: copi,
                  aliases: { name: d11, FunctionSpi: Mosi }     },
        Gpio12  { name: a4,
                  aliases: { name: gpio12, FunctionI2C: Sda}    },
        Gpio13  { name: a5,
                  aliases: { name: gpio13, FunctionI2C: Scl}    },
        Gpio15  { name: d3,
                  aliases: { name: gpio15 } },
        Gpio16  { name: d4,
                  aliases: { name: gpio16 } },
        Gpio17  { name: d5,
                  aliases: { name: gpio17 } },
        Gpio18  { name: d6,
                  aliases: { name: gpio18 } },
        Gpio19  { name: d7,
                  aliases: { name: gpio19 } },
        Gpio20  { name: d8,
                  aliases: { name: gpio20 } },
        Gpio21  { name: d9,
                  aliases: { name: gpio21 } },
        Gpio25  { name: d2,
                  aliases: { name: gpio25 } },
        Gpio26  { name: a0,
                  aliases: { name: gpio26 } },
        Gpio27  { name: a1,
                  aliases: { name: gpio27 } },
        Gpio28  { name: a2,
                  aliases: { name: gpio28 } },
        Gpio29  { name: a3,
                  aliases: { name: gpio29 } },
    );
}

pub struct Pins {
    pub gpio0: Pin<Gpio0, <Gpio0 as PinId>::Reset>,

}

pub const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
