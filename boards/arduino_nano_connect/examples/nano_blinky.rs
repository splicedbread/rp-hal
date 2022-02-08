//! # Nano Blinky Example
//!
//! Blinks the LED on a Nano board
//! Blinks the LED attached to gpio6, shared with the sck0 label

#![no_std]
#![no_main]

// the macro for the start up function
use cortex_m_rt::entry;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Time handling traits
use embedded_time::rate::*;

// Halt the program on panic (crate must be mentioned to be linked)
use panic_halt as _;

// Pull in important traits
use arduino_nano_connect::hal::prelude::*;

// A short alias for the Peripheral Access Crate, provides low-level reg access
use arduino_nano_connect::hal::pac;

// A short alias for the HAL, which provides higher-level drivers
use arduino_nano_connect::hal;

/// Entry point to bare-metal application
///
/// The `#[entry]` macro ensures the Cortex-M start-up code cals this function
/// after the global variables are initialized
///
/// The function follows the model of
/// 0. Setup
/// 1. Configure
/// 2. Initialize (if applicable)
/// 3. Loop
///
/// In specific it blinks the LED in an infinite loop
#[entry]
fn main() -> ! {
    // SETUP
    // Singleton opbjects from PAC so we can use peripheals
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // enable the watch dog timer, required by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // CONFIGURE
    // configure the clocks
    // by default, generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        arduino_nano_connect::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog
    )
    .ok()
    .unwrap();

    // the delay object allows for specified amounts of delay in ms
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // the single cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this nano board
    let pins = arduino_nano_connect::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS
    );

    // set the LED to be an output
    let mut led_pin = pins.sck0.into_push_pull_output();

    // LOOP
    // - Blink the LED at 1 Hz
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }

}
