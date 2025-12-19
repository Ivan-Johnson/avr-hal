pub use atmega_hal::port::{mode, Pin, PinMode, PinOps};

avr_hal_generic::renamed_pins! {
    /// Pins of the **Arduino Micro**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    ///
    /// The pins in this struct are in the same order as the schematic: first HEAD17-NOSS then HEAD17-NOSS-1
    ///
    /// * TODO: who cares?
    ///
    /// * TODO: is this actually a good idea? Maybe I should use a
    ///   more logical ordering instead, where all the DX pins are grouped together, etc.
    ///
    /// TODO: more/better docs. ref leonardo.rs.
    pub struct Pins {
        /// `MOSI`
        pub mosi: atmega_hal::port::PB2 = pb2,
        /// `RXLED` / `SS`
        pub led_rx: atmega_hal::port::PB0 = pb0,
        /// `D1` / `TX`
        pub d1: atmega_hal::port::PD3 = pd3,
        /// `D0` / `RX`
        pub d0: atmega_hal::port::PD2 = pd2,

        // `RESET`
        // `GND`

        /// `D2` / `SDA`
        pub d2: atmega_hal::port::PD1 = pd1,
        /// `D3` / `SCL`
        pub d3: atmega_hal::port::PD0 = pd0,
        /// `D4`
        pub d4: atmega_hal::port::PD4 = pd4,
        /// `D5`
        pub d5: atmega_hal::port::PC6 = pc6,
        /// `D6`
        pub d6: atmega_hal::port::PD7 = pd7,
        /// `D7`
        pub d7: atmega_hal::port::PE6 = pe6,
        /// `IO8`
        pub io8: atmega_hal::port::PB4 = pb4,
        /// `IO9`
        pub io9: atmega_hal::port::PB5 = pb5,
        /// `IO10`
        pub io10: atmega_hal::port::PB6 = pb6,
        /// `IO11`
        pub io11: atmega_hal::port::PB7 = pb7,
        /// `IO12`
        pub io12: atmega_hal::port::PD6 = pd6,









        /// `SCK`
        pub sck: atmega_hal::port::PB1 = pb1,
        /// `MISO`
        pub miso: atmega_hal::port::PB3 = pb3,

	// VIN
        // GND
        // RESET
        // +5V
        // NULL
        // NULL


        /// `A5`
        pub a5: atmega_hal::port::PF0 = pf0,
        /// `A4`
        pub a4: atmega_hal::port::PF1 = pf1,
        /// `A3`
        pub a3: atmega_hal::port::PF4 = pf4,
        /// `A2`
        pub a2: atmega_hal::port::PF5 = pf5,
        /// `A1`
        pub a1: atmega_hal::port::PF6 = pf6,
        /// `A0`
        pub a0: atmega_hal::port::PF7 = pf7,

        // AREF

        // 3V3

        /// `IO13`
        pub io13: atmega_hal::port::PC7 = pc7,
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = atmega_hal::Pins;
    }
}
