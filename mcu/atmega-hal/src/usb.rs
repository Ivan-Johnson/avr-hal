use avr_device::atmega32u4::PLL;
use avr_device::atmega32u4::USB_DEVICE;
use usb_device::bus::PollResult;
use usb_device::bus::UsbBus;
use usb_device::endpoint::EndpointAddress;
use usb_device::endpoint::EndpointType;
use usb_device::UsbDirection;
use usb_device::UsbError;

pub struct UsbdBus {}

impl UsbdBus {
	// TODO: I'm not sure that the arguments to the `new` function are
	// correct; there's a chance that they'll need to change during
	// implementation.
	pub fn new(_usb: USB_DEVICE, _pll: PLL) -> Self {
		todo!();
	}
}

impl UsbBus for UsbdBus {
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
