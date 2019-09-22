#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use stm32f1xx_hal::time::U32Ext;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::flash::FlashExt;
use nb::block;
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

use stm32f1xx_hal::{
    pac,
    timer::Timer,
};



use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = stm32f1xx_hal::delay::Delay::new(cp.SYST, clocks);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let rs = gpioa.pa10.into_push_pull_output(&mut gpioa.crh);
    let en = gpioa.pa9.into_push_pull_output(&mut gpioa.crh);
    let d4 = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let d5 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
    let d6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let d7 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);

    let mut lcd = HD44780::new_4bit(rs,en,d4,d5,d6,d7,delay);
    //let mut _timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    // Unshift display and set cursor to 0
    lcd.reset();

    // Clear existing characters
    lcd.clear();

    lcd.set_cursor_blink(CursorBlink::On);

    // Display the following string
    lcd.write_str("Hello, world!");

    // Move the cursor to the second line
    lcd.set_cursor_pos(40);

    // Display the following string on the second line
    lcd.write_str("I'm on line 2!");
    loop {

    }
}
