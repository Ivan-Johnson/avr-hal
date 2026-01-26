TODO: File a github issue on `Rahix/avr-device`

* This compiles successfully; I don't think that's expected. `set` should be
  tagged as `unsafe`?

  ```
  let usb = dp.USB_DEVICE;
  usb.uenum().write(|w| w.set(0xFF));
  ```

* XY problem:

  My actual goal is to remove ~all unsafes from my USB PR

  https://github.com/Rahix/avr-hal/pull/694
