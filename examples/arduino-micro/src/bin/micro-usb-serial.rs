#![no_std]
#![no_main]

use arduino_hal::pins;
use arduino_hal::prelude::*;
use arduino_hal::usb::UsbBus;
use arduino_hal::Peripherals;
use panic_halt as _;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbVidPid;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;
use usbd_serial::SerialPort;

#[arduino_hal::entry]
fn main() -> ! {
	let dp: Peripherals = Peripherals::take().unwrap();
	let pins = pins!(dp);
	let pll = dp.PLL;
	let usb = dp.USB_DEVICE;
	let mut serial_hw = arduino_hal::default_serial!(dp, pins, 57600);
	ufmt::uwriteln!(&mut serial_hw, "Hello from Arduino!\r").unwrap_infallible();

	// Configure PLL interface
	// prescale 16MHz crystal -> 8MHz
	pll.pllcsr().write(|w| w.pindiv().set_bit());
	// 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
	pll.pllfrq()
		.write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

	// Enable PLL
	pll.pllcsr().modify(|_, w| w.plle().set_bit());

	// Check PLL lock
	while pll.pllcsr().read().plock().bit_is_clear() {}

	let usb_bus = UsbBus::with_suspend_notifier(usb, pll);

	let mut serial_usb = SerialPort::new(&usb_bus);

	let string_descriptors = StringDescriptors::new(LangID::EN_US)
		.manufacturer("test manufacturer")
		.product("test product")
		.serial_number("test serial number");

	let mut usb_device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
		.strings(&[string_descriptors])
		.unwrap()
		.build();

	ufmt::uwriteln!(&mut serial_hw, "pre-loop").unwrap_infallible();

	let mut counter = 0;
	loop {
		counter += 1;
		ufmt::uwriteln!(&mut serial_hw, "Loop {}", counter).unwrap_infallible();
		if counter % 1_000 == 0 {
			let write_buf = [b'?'];
			serial_usb.write(&write_buf).unwrap();
		}

		usb_device.poll(&mut [&mut serial_usb]);
	}
}
