#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // IO pin 13 is connected to an onboard LED marked "L" (TODO: Is it actually marked L tho?)
    let mut led = pins.io13.into_output();

    loop {
        // One fast blink
        led.set_high();
        arduino_hal::delay_ms(100);
        led.toggle();
        arduino_hal::delay_ms(100);

        // One slow blink
        led.set_high();
        arduino_hal::delay_ms(800);
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
