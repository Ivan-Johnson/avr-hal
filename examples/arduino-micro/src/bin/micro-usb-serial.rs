//! A "Hello World" example that can be run on an Arduino Leonardo.
//!
//! # Usage
//!
//! 1. (Optional) Connect a pushbutton switch to the D2 pin of the Leonardo, and
//! connect the other pin of the switch to GND.
//!
//! 2. Connect the Leonardo to the computer with a USB cable.
//!
//! 3. Make sure [Ravedude](https://github.com/Rahix/avr-hal/tree/main/ravedude)
//! is installed. Then "run" the example to deploy it to the Arduino:
//!
//!   ```
//!   cargo run --release --example arduino_keyboard
//!   ```
//!
//! 4. Open Notepad (or whatever editor or text input of your choosing). Press
//! the button (or if you are not using one, short D2 to GND with a jumper). You
//! should see it type "Hello World"

#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]

use arduino_hal::usb::{SuspendNotifier, UsbBus};
use arduino_hal::{
	entry,
	pac::PLL,
	pins,
	port::{
		mode::{Input, PullUp},
		Pin,
	},
	Peripherals,
};
use panic_halt as _;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;
use usb_device::{
	class_prelude::UsbBusAllocator,
	device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::SerialPort;

const PAYLOAD: &[u8] = b"Hello World";

#[entry]
fn main() -> ! {
	let dp = Peripherals::take().unwrap();
	let pins = pins!(dp);
	let pll = dp.PLL;
	let usb = dp.USB_DEVICE;

	let status = pins.d13.into_output();
	let trigger = pins.d2.into_pull_up_input();

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

	let usb_bus = unsafe {
		static mut USB_BUS: Option<UsbBusAllocator<UsbBus<PLL>>> = None;
		&*USB_BUS.insert(UsbBus::with_suspend_notifier(usb, pll))
	};

	let mut serial = SerialPort::new(&usb_bus);

	let string_descriptors = StringDescriptors::new(LangID::EN_US)
		.manufacturer("test manufacturer")
		.product("test product")
		.serial_number("test serial number");

	let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
		.strings(&[string_descriptors])
		.unwrap()
		.build();

	unsafe {
		USB_CTX = Some(UsbContext {
			usb_device,
			serial,
			current_index: 0,
			pressed: false,
			trigger: trigger.downgrade(),
		});
	}

	// unsafe { interrupt::enable() };
	loop {
		// sleep();
		let ctx = unsafe { USB_CTX.as_mut().unwrap() };
		ctx.poll();
	}
}

static mut USB_CTX: Option<UsbContext<PLL>> = None;

struct UsbContext<S: SuspendNotifier> {
	usb_device: UsbDevice<'static, UsbBus<S>>,
	serial: SerialPort<'static>,
	current_index: usize,
	pressed: bool,
	trigger: Pin<Input<PullUp>>,
}

impl<S: SuspendNotifier> UsbContext<S> {
	fn poll(&mut self) {
		let write_buf = [b'?'; 20];
		self.serial.write(&write_buf).unwrap();

		if self.usb_device.poll(&mut [&mut self.serial]) {
			let mut report_buf = [0u8; 1];

			self.serial.pull_raw_output(&mut report_buf);
		}
	}
}
