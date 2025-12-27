use avr_device::atmega32u4::USB_DEVICE;
use ::usb_device::bus::PollResult;
use ::usb_device::bus::UsbBus;
use ::usb_device::endpoint::EndpointAddress;
use ::usb_device::endpoint::EndpointType;
use ::usb_device::UsbDirection;
use ::usb_device::UsbError;

// TODO: rename. What are the naming conventions anyways?
pub struct MyUsbBus {}

impl MyUsbBus {
	pub fn new(_usb: USB_DEVICE) -> Self {
		// TODO: Is there anything else that we should be taking
		// ownership of, besides `USB_DEVICE`?
		// (CHECK https://github.com/agausmann/atmega-usbd/blob/master/src/lib.rs)
		//
		// * I think we might need an immutable reference to `PLL` in
		//   order to ensure that the clock speed doesn't change?
		//
		// * There are five USB-related pins on the AVR.
		//
		//   None of them are used for anything other than USB, so we
		//   don't need to take ownership of them. Probably. Plus I
		//   can't find anything about d+ or d- in
		//   `./target/debug/build/avr-device-28b02c0ef1e86e40/out/pac/atmega32u4.rs`.
		//
		//   * VBUS
		//
		//   * D+
		//
		//   * D-
		//
		//   * UGRND
		//
		//   * UID: Is this even a real pin???
		//
		//     This appears in section 21.3.1 of the ATmega16U4/32U4
		//     datasheet ("Bus Powered Device"). I cannot find it
		//     referenced anywhere else. In particular, it doesn't seem
		//     to appear in section 1 ("Pin Configurations").
		todo!();
	}
}

// TODO:
//
// * Read the docs for this trait:
//
//   https://github.com/rust-embedded-community/usb-device/blob/master/src/bus.rs#L11
//
// * Read `musb`'s implementation of this trait, and use it as reference
//
//   https://github.com/decaday/musb/blob/a9b9c5487127d03408d479aeda6f6027e801f9a5/src/usb_device_impl/mod.rs#L36-L467
impl UsbBus for MyUsbBus {
	fn alloc_ep(
		&mut self,
		_: UsbDirection,
		_: Option<EndpointAddress>,
		_: EndpointType,
		_: u16,
		_: u8,
	) -> Result<EndpointAddress, UsbError> {
		todo!()
	}
	fn enable(&mut self) {
		todo!()
	}
	fn reset(&self) {
		todo!()
	}
	fn set_device_address(&self, _: u8) {
		todo!()
	}
	fn write(&self, _: EndpointAddress, _: &[u8]) -> Result<usize, UsbError> {
		todo!()
	}
	fn read(&self, _: EndpointAddress, _: &mut [u8]) -> Result<usize, UsbError> {
		todo!()
	}
	fn set_stalled(&self, _: EndpointAddress, _: bool) {
		todo!()
	}
	fn is_stalled(&self, _: EndpointAddress) -> bool {
		todo!()
	}
	fn suspend(&self) {
		todo!()
	}
	fn resume(&self) {
		todo!()
	}
	fn poll(&self) -> PollResult {
		todo!()
	}
}
