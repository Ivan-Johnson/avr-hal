/*!
 *  A Hello World CLI, over USB.
 *
 *  If you connect the board to a computer via USB, the board will show up as a
 *  keyboard. The keyboard will automatically type the message "Hello, World"
 *  once. You can have it type the message again by rebooting the board, using
 *  the built-in reset button.
 */
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
	// This example is more complicate than `micro-usb-serial.rs`, if only
	// slightly [1].
	//
	// As such, I'll start by implementing the serial support.
	//
	// Eventually, I'd like to create an example showing how the Arduino
	// could be used as a keyboard and/or mouse.
	//
	// For this, we'd need to use an USB HID (human interface device)
	// class. There are two such classes listed in usb-device's README:
	// * https://github.com/twitchyliquid64/usbd-hid
	// * https://github.com/dlkj/usbd-human-interface-device
	//
	// I don't know which of the two we should use. For now, I'm just going
	// to ignore this problem and focus on `micro-usb-serial.rs` instead.
	todo!();
}
