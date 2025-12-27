/*!
 *  A Hello World CLI, over USB.
 */
#![no_std]
#![no_main]
use arduino_hal::Peripherals;
use panic_halt as _;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::StringDescriptors;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbVidPid;
use usb_device::LangID;
use usbd_serial::SerialPort;

#[arduino_hal::entry]
fn main() -> ! {
	let dp: Peripherals = arduino_hal::Peripherals::take().unwrap();
	let usb_bus = arduino_hal::default_usb_bus(dp.USB_DEVICE);
	let usb_bus_allocator = UsbBusAllocator::new(usb_bus);
	let mut serial = SerialPort::new(&usb_bus_allocator);

	let string_descriptors = StringDescriptors::new(LangID::EN_US)
		.manufacturer("test manufacturer")
		.product("test product")
		.serial_number("test serial number");

	let rand_ids = UsbVidPid(0x1ea7, 0x4a09);

	let mut usb_dev = UsbDeviceBuilder::new(&usb_bus_allocator, rand_ids)
		.strings(&[string_descriptors])
		.unwrap()
		.max_packet_size_0(64)
		.unwrap()
		.device_class(usbd_serial::USB_CLASS_CDC)
		.build();

	loop {
		// Wait until we have data
		if !usb_dev.poll(&mut [&mut serial]) {
			continue;
		}

		// Read the data into this buffer
		let mut read_buf = [0u8; 10];
		let Ok(read_count) = serial.read(&mut read_buf) else {
			continue;
		};
		if read_count == 0 {
			continue;
		}

		// Ideally we want to do something like this:
		//
		// ```
		// let mut write_buf = [0u8; 20];
		// let write_count = ufmt::uwriteln!(&mut write_buf, "Got: {:?}", read_buf);
		// ```
		//
		// TODO: get the above to actually compile. In the meantime,
		// we simply return an equal number of question marks.
		let write_buf = [b'?'; 20];
		let write_count = read_count;

		// TODO: is this `.expect()` correct?
		let len = serial
			.write(&write_buf[0..write_count])
			.expect("The host should be reading data faster than the arduino can write it");
		assert_eq!(len, write_count);
	}
}
