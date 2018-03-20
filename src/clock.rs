extern crate cortex_m;

use mkw41z::{MCG, RSIM, SIM};

pub fn clocks() -> Clocks {
        // Turn on clocks for the XCVR
        // Enable RF OSC in RSIM and wait for ready
        let rsim = unsafe { &(*RSIM::ptr()) };
        rsim.control.modify(|_, w| w.rf_osc_en()._0001());

        // Prevent XTAL_OUT_EN from generating XTAL_OUT request
        rsim.rf_osc_ctrl
            .modify(|_, w| w.radio_ext_osc_ovrd_en().set_bit());

        // Wait for RF_OSC_READY
        while !rsim.control.read().rf_osc_ready().bit_is_set() {}

        // Set clock into a safe frequency range
        let sim = unsafe { &(*SIM::ptr()) };
        sim.clkdiv1.write(|w| w.outdiv4().bits(0x4));

        let mcg = unsafe { &(*MCG::ptr()) };

        // External frequency 32_000_000, i.e. larger 8_000_000
        mcg.c2.modify(|_, w| unsafe { w.range().bits(2) });
        mcg.c7.modify(|_, w| w.oscsel()._0());

        // Set CLKS and IREFS
        mcg.c1
            .modify(|_, w| w.clks()._00().frdiv()._101().irefs()._0());

        // If use external crystal as clock source, wait for it stable.
        if mcg.c2.read().erefs().bit() {
            while !mcg.s.read().oscinit0().bit() {}
        }

        // Wait and check status
        while !mcg.s.read().irefst().bit() {}

        // Set DRS and DMX32
        mcg.c4.modify(|_, w| w.dmx32()._0().drst_drs()._01());

        /* Wait for DRST_DRS update. */
        while mcg.c4.read().drst_drs().bits() != 1 {}

        // Wait and check clock status
        while mcg.s.read().clkst().bits() != 0 {}

        // Wait for FLL stable time
        for _ in 0..30_000 {
            cortex_m::asm::nop();
        }

        // Do we need to update FCRDIV?
        if mcg.sc.read().fcrdiv().bits() != 0 {
            /* If fast IRC is in use currently, change to slow IRC. */
            let mcgs = mcg.s.read();
            if mcgs.ircst().bit() && (mcgs.clkst().bits() == 1 || mcg.c1.read().irclken().bit()) {
                mcg.c2.modify(|_, w| w.ircs().clear_bit());
                while mcg.c2.read().ircs().bit() {}
            }
            // Update FCRDIV
            mcg.sc
                .modify(|_, w| w.fcrdiv()._000().atmf().clear_bit().locs0().clear_bit());
        }

        // Set internal reference clock selection
        mcg.c2.modify(|_, w| w.ircs().clear_bit());
        mcg.c1
            .modify(|_, w| w.irclken().set_bit().irefsten().clear_bit());

        // If MCGIRCLK is used, need to wait for MCG_S_IRCST
        while mcg.s.read().ircst().bit() {}

        sim.clkdiv1.modify(|_, w| w.outdiv4().bits(0x1));
        sim.sopt1.modify(|_, w| w.osc32ksel()._00());

        Clocks {
            coreclk: 40_000_000,
            oscextclk: 32_000_000,
            systemclk: 40_000_000,
            busclk: 20_000_000,
            flashclk: 20_000_000,
        }
}

#[derive(Clone, Copy)]
pub struct Clocks {
    coreclk: u32,
    oscextclk: u32,
    systemclk: u32,
    busclk: u32,
    flashclk: u32,
}

impl Clocks {
    pub fn coreclk(&self) -> u32 {
        self.coreclk
    }

    pub fn oscextclk(&self) -> u32 {
        self.oscextclk
    }

    pub fn systemclk(&self) -> u32 {
        self.systemclk
    }

    pub fn busclk(&self) -> u32 {
        self.busclk
    }

    pub fn flashclk(&self) -> u32 {
        self.flashclk
    }
}
