//! [`Instrument`] implementations for GPIO pins.
//!
//! Types implementing `embedded-hal v1.0`
//! [`OutputPin`](embedded_hal_1::digital::OutputPin) and `embedded-hal v0.2`
//! [`OutputPin`](embedded_hal_0_2::digital::v2::OutputPin) are supported.

use crate::Instrument;
use embedded_hal_1 as embedded_hal;

#[cfg(feature = "embedded-hal_0_2")]
use embedded_hal_0_2 as embedded_hal_legacy;

/// [`Instrument`] implementation for an [`OutputPin`] (`embedded-hal` version
/// 1.0).
///
/// This type is also available as [`GpioRef`], which does not consume the
/// underlying [`OutputPin`]. For an [`Instrument`] implementation which
/// supports `embedded-hal` version 0.2, see [`LegacyGpio`].
///
/// [`OutputPin`]: embedded_hal::digital::OutputPin
pub struct Gpio<P: embedded_hal::digital::OutputPin> {
    pin: P,
}

impl<P: embedded_hal::digital::OutputPin> From<P> for Gpio<P> {
    #[inline]
    fn from(pin: P) -> Self {
        Self::new(pin)
    }
}

impl<P: embedded_hal::digital::OutputPin> Gpio<P> {
    /// Create a new [`Gpio`].
    #[inline]
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Return the underlying [`OutputPin`](embedded_hal::digital::OutputPin).
    #[inline]
    pub fn free(self) -> P {
        self.pin
    }
}

impl<P: embedded_hal::digital::OutputPin> Instrument for Gpio<P> {
    #[inline]
    fn on_enter(&mut self) {
        let _ = self.pin.set_high();
    }

    #[inline]
    fn on_exit(&mut self) {
        let _ = self.pin.set_low();
    }
}

/// Reference-taking version of an [`Instrument`] implementation for an
/// [`OutputPin`] (`embedded-hal` version 1.0).
///
/// This type is also available as [`Gpio`], which does consumes the
/// underlying [`OutputPin`]. For an [`Instrument`] implementation which
/// supports `embedded-hal` version 0.2, see [`LegacyGpioRef`].
///
/// [`OutputPin`]: embedded_hal::digital::OutputPin
pub struct GpioRef<'a, P: embedded_hal::digital::OutputPin> {
    pin: &'a mut P,
}

impl<'a, P: embedded_hal::digital::OutputPin> GpioRef<'a, P> {
    /// Create a new [`GpioRef`].
    #[inline]
    pub fn new(pin: &'a mut P) -> Self {
        Self { pin }
    }
}

impl<'a, P: embedded_hal::digital::OutputPin> Instrument for GpioRef<'a, P> {
    #[inline]
    fn on_enter(&mut self) {
        let _ = self.pin.set_high();
    }

    #[inline]
    fn on_exit(&mut self) {
        let _ = self.pin.set_low();
    }
}

/// [`Instrument`] implementation for an [`OutputPin`] (`embedded-hal` version
/// 0.2).
///
/// This type is also available as [`LegacyGpioRef`], which does not consume the
/// underlying [`OutputPin`]. For an [`Instrument`] implementation which
/// supports `embedded-hal` version 1.0, see [`Gpio`].
///
/// [`OutputPin`]: embedded_hal::digital::OutputPin
#[cfg(feature = "embedded-hal_0_2")]
pub struct LegacyGpio<P: embedded_hal_legacy::digital::v2::OutputPin> {
    pin: P,
}

#[cfg(feature = "embedded-hal_0_2")]
impl<P: embedded_hal_legacy::digital::v2::OutputPin> From<P> for LegacyGpio<P> {
    fn from(pin: P) -> Self {
        Self { pin }
    }
}

#[cfg(feature = "embedded-hal_0_2")]
impl<P: embedded_hal_legacy::digital::v2::OutputPin> LegacyGpio<P> {
    /// Create a new [`LegacyGpio`].
    #[inline]
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Return the underlying
    /// [`OutputPin`](embedded_hal_legacy::digital::v2::OutputPin).
    #[inline]
    pub fn free(self) -> P {
        self.pin
    }
}

#[cfg(feature = "embedded-hal_0_2")]
impl<P: embedded_hal_legacy::digital::v2::OutputPin> Instrument for LegacyGpio<P> {
    #[inline]
    fn on_enter(&mut self) {
        let _ = self.pin.set_high();
    }

    #[inline]
    fn on_exit(&mut self) {
        let _ = self.pin.set_low();
    }
}

/// Reference-taking version of an [`Instrument`] implementation for an
/// [`OutputPin`] (`embedded-hal` version 0.2).
///
/// This type is also available as [`LegacyGpio`], which consumes the
/// underlying [`OutputPin`]. For an [`Instrument`] implementation which
/// supports `embedded-hal` version 1.0, see [`GpioRef`].
///
/// [`OutputPin`]: embedded_hal::digital::OutputPin
#[cfg(feature = "embedded-hal_0_2")]
pub struct LegacyGpioRef<'a, P: embedded_hal_legacy::digital::v2::OutputPin> {
    pin: &'a mut P,
}

#[cfg(feature = "embedded-hal_0_2")]
impl<'a, P: embedded_hal_legacy::digital::v2::OutputPin> LegacyGpioRef<'a, P> {
    /// Create a new [`LegacyGpioRef`].
    #[inline]
    pub fn new(pin: &'a mut P) -> Self {
        Self { pin }
    }
}

#[cfg(feature = "embedded-hal_0_2")]
impl<'a, P: embedded_hal_legacy::digital::v2::OutputPin> Instrument for LegacyGpioRef<'a, P> {
    #[inline]
    fn on_enter(&mut self) {
        let _ = self.pin.set_high();
    }

    #[inline]
    fn on_exit(&mut self) {
        let _ = self.pin.set_low();
    }
}
