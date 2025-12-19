#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::Timer4Pwm;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer4 = Timer4Pwm::new(dp.TC4, Prescaler::Prescale64);

    let mut pwm_led = pins.io13.into_output().into_pwm(&timer4);
    pwm_led.enable();

    loop {
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            arduino_hal::delay_ms(10);
        }
    }
}
