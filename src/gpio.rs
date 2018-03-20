//! General Purpose Input / Output

use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

pub struct AF0;
pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;
pub struct AF5;
pub struct AF6;
pub struct AF7;

pub struct Alternate<MODE> {
    _mode: PhantomData<MODE>,
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;

macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $iopxenr:ident, $PXx:ident, $PCFGx:ident [
        $($PXi:ident: ($pxi:ident, $pcrx:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;

            use hal::digital::{InputPin, OutputPin};
            use mkw41z::{$GPIOX, $PCFGx};

            use mkw41z::SIM;
            use super::{
                Alternate, Floating, GpioExt, Input, Output,
                PullDown, PullUp, PushPull, AF0, AF1, AF2, AF3, AF4, AF5, AF6, AF7,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    // NOTE(unsafe) This executes only during initialisation
                    let sim = unsafe { &(*SIM::ptr()) };
                    sim.scgc5.modify(|_, w| w.$iopxenr().set_bit());

                    Parts {
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).pdir.read().bits() & (1 << self.i) == 0 }
                }

                fn set_high(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).psor.write(|w| w.bits(1 << self.i)) }
                }

                fn set_low(&mut self) {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).pcor.write(|w| w.bits(1 << self.i)) }
                }
            }

            impl<MODE> InputPin for $PXx<Input<MODE>> {
                fn is_high(&self) -> bool {
                    !self.is_low()
                }

                fn is_low(&self) -> bool {
                    // NOTE(unsafe) atomic read with no side effects
                    unsafe { (*$GPIOX::ptr()).pdir.read().bits() & (1 << self.i) == 0 }
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate in AF0 mode
                    pub fn into_alternate_af0(
                        self,
                    ) -> $PXi<Alternate<AF0>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._000() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF1 mode
                    pub fn into_alternate_af1(
                        self,
                    ) -> $PXi<Alternate<AF1>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._001() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF2 mode
                    pub fn into_alternate_af2(
                        self,
                    ) -> $PXi<Alternate<AF2>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._010() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF3 mode
                    pub fn into_alternate_af3(
                        self,
                    ) -> $PXi<Alternate<AF3>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._011() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF4 mode
                    pub fn into_alternate_af4(
                        self,
                    ) -> $PXi<Alternate<AF4>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._100() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF5 mode
                    pub fn into_alternate_af5(
                        self,
                    ) -> $PXi<Alternate<AF5>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._101() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF6 mode
                    pub fn into_alternate_af6(
                        self,
                    ) -> $PXi<Alternate<AF6>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._110() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate in AF7 mode
                    pub fn into_alternate_af7(
                        self,
                    ) -> $PXi<Alternate<AF7>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.modify(|_, w| { w.mux()._111() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                    ) -> $PXi<Input<Floating>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.write(|w| { w.mux()._001() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                        ) -> $PXi<Input<PullDown>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.write(|w| {
                                w.mux()._001().ps()._0().pe().set_bit() });
                        }
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                    ) -> $PXi<Input<PullUp>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.write(|w| {
                                w.mux()._001().ps()._1().pe().set_bit() });
                        }

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    pub fn into_push_pull_output(
                        self,
                    ) -> $PXi<Output<PushPull>> {
                        unsafe {
                            &(*$PCFGx::ptr()).$pcrx.write(|w| { w.mux()._001() });
                            &(*$GPIOX::ptr()).pddr.modify(|r, w| { w.bits(r.bits() | (0b1 <<
                                                                                      $i))});
                        }

                        $PXi { _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).pdir.read().bits() & (1 << $i) == 0 }
                    }

                    fn set_high(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).psor.write(|w| w.bits(1 << $i)) }
                    }

                    fn set_low(&mut self) {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).pcor.write(|w| w.bits(1 << $i)) }
                    }
                }

                impl<MODE> $PXi<Input<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Input<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    fn is_high(&self) -> bool {
                        !self.is_low()
                    }

                    fn is_low(&self) -> bool {
                        // NOTE(unsafe) atomic read with no side effects
                        unsafe { (*$GPIOX::ptr()).pdir.read().bits() & (1 << $i) == 0 }
                    }
                }
            )+

                impl<TYPE> $PXx<TYPE> {
                    pub fn get_id (&self) -> u8
                    {
                        self.i
                    }
                }
        }
    }
}

gpio!(GPIOA, gpioa, porta, PTA, PORTA [
    PTA0: (pta0, pcr0, 0, Input<Floating>),
    PTA1: (pta1, pcr1, 1, Input<Floating>),
    PTA2: (pta2, pcr2, 2, Input<Floating>),
    PTA16: (pta16, pcr16, 16, Input<Floating>),
    PTA17: (pta17, pcr17, 17, Input<Floating>),
    PTA18: (pta18, pcr18, 18, Input<Floating>),
    PTA19: (pta19, pcr19, 19, Input<Floating>),
]);

gpio!(GPIOB, gpiob, portb, PTB, PORTB [
    PTB0: (ptb0, pcr0, 0, Input<Floating>),
    PTB1: (ptb1, pcr1, 1, Input<Floating>),
    PTB2: (ptb2, pcr2, 2, Input<Floating>),
    PTB16: (ptb16, pcr16, 16, Input<Floating>),
    PTB17: (ptb17, pcr17, 17, Input<Floating>),
    PTB18: (ptb18, pcr18, 18, Input<Floating>),
]);

gpio!(GPIOC, gpioc, portc, PTC, PORTC [
    PTC1: (ptc1, pcr1, 1, Input<Floating>),
    PTC2: (ptc2, pcr2, 2, Input<Floating>),
    PTC3: (ptc3, pcr3, 3, Input<Floating>),
    PTC4: (ptc4, pcr4, 4, Input<Floating>),
    PTC5: (ptc5, pcr5, 5, Input<Floating>),
    PTC6: (ptc6, pcr6, 6, Input<Floating>),
    PTC7: (ptc7, pcr7, 7, Input<Floating>),
    PTC16: (ptc16, pcr16, 16, Input<Floating>),
    PTC17: (ptc17, pcr17, 17, Input<Floating>),
    PTC18: (ptc18, pcr18, 18, Input<Floating>),
    PTC19: (ptc19, pcr19, 19, Input<Floating>),
]);
