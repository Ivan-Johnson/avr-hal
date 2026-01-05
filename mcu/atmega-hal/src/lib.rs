#![no_std]
#![feature(asm_experimental_arch)]

//! `atmega-hal`
//! =============
//! Common HAL (hardware abstraction layer) for ATmega* microcontrollers.
//!
//! **Note**: This version of the documentation was built for
#![cfg_attr(feature = "atmega48p", doc = "**ATmega48P**.")]
#![cfg_attr(feature = "atmega16", doc = "**ATmega16**.")]
#![cfg_attr(feature = "atmega164pa", doc = "**ATmega164PA**.")]
#![cfg_attr(feature = "atmega168", doc = "**ATmega168**.")]
#![cfg_attr(feature = "atmega328p", doc = "**ATmega328P**.")]
#![cfg_attr(feature = "atmega328pb", doc = "**ATmega328PB**.")]
#![cfg_attr(feature = "atmega32a", doc = "**ATmega32a**.")]
#![cfg_attr(feature = "atmega32u4", doc = "**ATmega32U4**.")]
#![cfg_attr(feature = "atmega2560", doc = "**ATmega2560**.")]
#![cfg_attr(feature = "atmega128a", doc = "**ATmega128A**.")]
#![cfg_attr(feature = "atmega1280", doc = "**ATmega1280**.")]
#![cfg_attr(feature = "atmega1284p", doc = "**ATmega1284P**.")]
#![cfg_attr(feature = "atmega8", doc = "**ATmega8**.")]
#![cfg_attr(feature = "atmega88p", doc = "**ATmega88P**.")]
//! This means that only items which are available for this MCU are visible.  If you are using
//! a different chip, try building the documentation locally with:
//!
//! ```text
//! cargo doc --features <your-mcu> --open
//! ```

#[cfg(all(
	not(feature = "device-selected"),
	not(feature = "disable-device-selection-error")
))]
compile_error!(
	"This crate requires you to specify your target chip as a feature.

    Please select one of the following

    * atmega48p
    * atmega16
    * atmega164pa
    * atmega168
    * atmega328p
    * atmega328pb
    * atmega32u4
    * atmega128a
    * atmega1280
    * atmega2560
    * atmega1284p
    * atmega8
    * atmega88p
    "
);

/// Reexport of `atmega1280` from `avr-device`
///
#[cfg(feature = "atmega1280")]
pub use avr_device::atmega1280 as pac;
/// Reexport of `atmega1284p` from `avr-device`
///
#[cfg(feature = "atmega1284p")]
pub use avr_device::atmega1284p as pac;
/// Reexport of `atmega128a` from `avr-device`
///
#[cfg(feature = "atmega128a")]
pub use avr_device::atmega128a as pac;
/// Reexport of `atmega16` from `avr-device`
///
#[cfg(feature = "atmega16")]
pub use avr_device::atmega16 as pac;
/// Reexport of `atmega164pa` from `avr-device`
///
#[cfg(feature = "atmega164pa")]
pub use avr_device::atmega164pa as pac;
/// Reexport of `atmega168` from `avr-device`
///
#[cfg(feature = "atmega168")]
pub use avr_device::atmega168 as pac;
/// Reexport of `atmega2560` from `avr-device`
///
#[cfg(feature = "atmega2560")]
pub use avr_device::atmega2560 as pac;
/// Reexport of `atmega328p` from `avr-device`
///
#[cfg(feature = "atmega328p")]
pub use avr_device::atmega328p as pac;
/// Reexport of `atmega328pb` from `avr-device`
///
#[cfg(feature = "atmega328pb")]
pub use avr_device::atmega328pb as pac;
/// Reexport of `atmega32a` from `avr-device`
///
#[cfg(feature = "atmega32a")]
pub use avr_device::atmega32a as pac;
/// Reexport of `atmega32u4` from `avr-device`
///
#[cfg(feature = "atmega32u4")]
pub use avr_device::atmega32u4 as pac;
/// Reexport of `atmega48p` from `avr-device`
///
#[cfg(feature = "atmega48p")]
pub use avr_device::atmega48p as pac;
/// Reexport of `atmega8` from `avr-device`
///
#[cfg(feature = "atmega8")]
pub use avr_device::atmega8 as pac;
/// Reexport of `atmega88p` from `avr-device`
///
#[cfg(feature = "atmega88p")]
pub use avr_device::atmega88p as pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use avr_device::entry;

#[cfg(feature = "device-selected")]
pub use pac::Peripherals;

pub use avr_hal_generic::clock;
pub use avr_hal_generic::delay;
pub use avr_hal_generic::prelude;

#[cfg(feature = "atmega32u4")]
mod usb;

/// This module does X, Y, and Z.
///
/// Foo, bar, baz.
///
/// Goodbye, world.
///
/// # Limitations
///
/// ## Limitation: Timers
///
/// We do not allow hardware timers to be used simultanously with UsbdBus. We enforce this by
/// setting PLLTM to zero, which disconnects the timers from the PLL clock output.
///
/// It's absolutely possible to use both at the same time, we just haven't yet implemented a safe
/// wrapper to deal with these complexities. For details, see GitHub issue #TBD.
///
/// TODO make a github issue:
///
/// * The coplexities involved are:
///
///   * We need some way to ensure that the PLL configuration is compatible with both the timer and
///     the USB modules. Any time the PLL configuration changes, we similarly need to ensure that the
///     USB and timer modules are updated appropriately.
///
///   * When the USB module is suspended, the PLL output clock is stopped. We need to ensure that
///     this doesn't break the user's timer code.
///
///  * Possible solutions:
///
///    * For the first issue:
///
///      Create a `setup_pll(pll: &mut PLL)` function that configures the PLL.
///
///      Create a new constructor for `UsbdBus` that takes `&PLL` as input, instead of `PLL`.
///
///      Add a lifetime parameter to `UsbdBus`. This will prevent the user from modifying the PLL while `UsbdBus` exists.
///
///    * For the second issue:
///
///      We could do basically the exact same thing that `agausmann/atmega-usbd` does:
///      https://github.com/agausmann/atmega-usbd/blob/master/src/lib.rs#L590-L618
///
///      This essentially just defines a `suspend` and `resume` callback functions,
///      which the user can implement however they want.
///
///      The default implementation would do basically the exact same thing that we
///      do today: take ownership of `PLL` and disable the hardware timers.
///
///    Note that the solution to the first issue requires a persistent immutable reference
///    to PLL, while the solution to the second issue requires a persistent mutable
///    reference to PLL. It is not yet certain whether or not they are using the same fields
///    of PLL. If so, this will be a problem.
///
/// ## Limitation: Power Usage
///
/// The current implementation does not attempt to minimize power usage. For details, see GitHub issue #TBD.
///
/// TODO: make a github issue:
///
/// * Add support for using interrupts, similar to `agausmann/atmega-usbd`
///
/// * Shutdown the PLL when the USB module is suspended
///
/// * ???
#[cfg(feature = "atmega32u4")]
pub fn default_usb_bus_with_pll(usb: avr_device::atmega32u4::USB_DEVICE, pll: avr_device::atmega32u4::PLL) -> impl usb_device::class_prelude::UsbBus {
	return usb::UsbdBus::new(usb, pll);
}

/// This macro is exactly equivalent to `default_usb_bus_with_pll`.
#[cfg(feature = "atmega32u4")]
#[macro_export]
macro_rules! default_usb_bus_with_pll_macro {
       ($p:expr) => {
               $crate::default_usb_bus_with_pll($p.USB_DEVICE, $p.PLL)
       };
}

#[cfg(feature = "device-selected")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub use adc::Adc;

#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(feature = "device-selected")]
pub use i2c::I2c;

#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub use spi::Spi;

#[cfg(feature = "device-selected")]
pub mod port;
#[cfg(feature = "device-selected")]
pub use port::Pins;

#[cfg(feature = "device-selected")]
pub mod simple_pwm;

#[cfg(feature = "device-selected")]
pub mod usart;
#[cfg(feature = "device-selected")]
pub use usart::Usart;

#[cfg(feature = "device-selected")]
pub mod wdt;
#[cfg(feature = "device-selected")]
pub use wdt::Wdt;

#[cfg(feature = "device-selected")]
pub mod eeprom;
#[cfg(feature = "device-selected")]
pub use eeprom::Eeprom;

pub struct Atmega;

#[cfg(any(
	feature = "atmega48p",
	feature = "atmega88p",
	feature = "atmega168",
	feature = "atmega328p"
))]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
	};
}
#[cfg(any(feature = "atmega16", feature = "atmega164pa"))]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
	};
}
#[cfg(feature = "atmega328pb")]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE)
	};
}
#[cfg(feature = "atmega32u4")]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF)
	};
}

#[cfg(any(feature = "atmega128a"))]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new(
			$p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF, $p.PORTG,
		)
	};
}

#[cfg(any(feature = "atmega1280", feature = "atmega2560"))]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new(
			$p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD, $p.PORTE, $p.PORTF, $p.PORTG, $p.PORTH, $p.PORTJ,
			$p.PORTK, $p.PORTL,
		)
	};
}

#[cfg(any(feature = "atmega1284p", feature = "atmega32a"))]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTC, $p.PORTD)
	};
}

#[cfg(any(feature = "atmega8"))]
#[macro_export]
macro_rules! pins {
	($p:expr) => {
		$crate::Pins::new($p.PORTB, $p.PORTC, $p.PORTD)
	};
}
