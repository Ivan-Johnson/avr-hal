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
	ufmt::uwriteln!(&mut serial_hw, "Hello from Arduino!").unwrap_infallible();

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



	let mut counter = 0;
	loop {
		counter += 1;
		if counter % 50_000 == 0 {
			ufmt::uwriteln!(&mut serial_hw, "{} loops with nothing to do", counter).unwrap_infallible();
		}

		// Wait until we have data
		if !usb_device.poll(&mut [&mut serial_usb]) {
			continue;
		}
		counter = 0;

		// Read the data into this buffer
		let mut read_buf = [0u8; 10];
		let Ok(read_count) = serial_usb.read(&mut read_buf) else {
			ufmt::uwriteln!(&mut serial_hw, "serial read failed??").unwrap_infallible();
			continue;
		};
		if read_count == 0 {
			ufmt::uwriteln!(&mut serial_hw, "serial read returned no data").unwrap_infallible();
			continue;
		}
		ufmt::uwriteln!(&mut serial_hw, "serial read returned:").unwrap_infallible();
		for byte in read_buf {
			ufmt::uwrite!(&mut serial_hw, "{}, ", byte).unwrap_infallible();
		}
		ufmt::uwriteln!(&mut serial_hw, "").unwrap_infallible();


		// Ideally we want to do something like this:
		//
		// ```
		// let mut write_buf = [0u8; 20];
		// let write_count = ufmt::uwriteln!(&mut write_buf, "Got: {}", &write_buf);
		// ```
		//
		// TODO: Figure out how to get the above code to compile. It seems like
		// I might need to manually implement the uDebug trait? That doesn't seem
		// right... In the meantime, simply echo the string back

		// TODO: is this `.expect()` safe?
		let len = serial_usb
			.write(&read_buf[0..read_count])
			.expect("The host should be reading data faster than the arduino can write it");
		assert_eq!(len, read_count);
	}
}
