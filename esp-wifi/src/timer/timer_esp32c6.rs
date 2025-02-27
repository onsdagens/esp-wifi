use crate::{
    binary,
    hal::{
        interrupt,
        macros::interrupt,
        peripherals::{self, Interrupt},
    },
};

pub fn setup_radio_isr() {
    #[cfg(feature = "wifi")]
    {
        unwrap!(interrupt::enable(
            Interrupt::WIFI_MAC,
            interrupt::Priority::Priority1
        ));
        unwrap!(interrupt::enable(
            Interrupt::WIFI_PWR,
            interrupt::Priority::Priority1
        ));
    }

    // make sure to disable WIFI_BB/MODEM_PERI_TIMEOUT by mapping it to CPU interrupt 31 which is masked by default
    // for some reason for this interrupt, mapping it to 0 doesn't deactivate it
    let interrupt_core0 = unsafe { &*peripherals::INTERRUPT_CORE0::PTR };
    interrupt_core0
        .wifi_bb_intr_map
        .write(|w| w.wifi_bb_intr_map().variant(31));
    interrupt_core0
        .modem_peri_timeout_intr_map
        .write(|w| w.modem_peri_timeout_intr_map().variant(31));

    #[cfg(feature = "ble")]
    {
        unwrap!(interrupt::enable(
            Interrupt::LP_TIMER,
            interrupt::Priority::Priority1
        ));
        unwrap!(interrupt::enable(
            Interrupt::BT_MAC,
            interrupt::Priority::Priority1
        ));
    }
}

#[cfg(feature = "wifi")]
#[interrupt]
fn WIFI_MAC() {
    unsafe {
        let (fnc, arg) = crate::wifi::os_adapter::ISR_INTERRUPT_1;

        trace!("interrupt WIFI_MAC {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            let fnc: fn(*mut binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }

        trace!("interrupt 1 done");
    };
}

#[cfg(feature = "wifi")]
#[interrupt]
fn WIFI_PWR() {
    unsafe {
        let (fnc, arg) = crate::wifi::os_adapter::ISR_INTERRUPT_1;

        trace!("interrupt WIFI_PWR {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            let fnc: fn(*mut binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }

        trace!("interrupt 1 done");
    };
}

#[cfg(feature = "ble")]
#[interrupt]
fn LP_TIMER() {
    unsafe {
        trace!("LP_TIMER interrupt");

        let (fnc, arg) = crate::ble::npl::ble_os_adapter_chip_specific::ISR_INTERRUPT_7;

        trace!("interrupt LP_TIMER {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            trace!("interrupt LP_TIMER call");

            let fnc: fn(*mut binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
            trace!("LP_TIMER done");
        }

        trace!("interrupt LP_TIMER done");
    };
}

#[cfg(feature = "ble")]
#[interrupt]
fn BT_MAC() {
    unsafe {
        trace!("BT_MAC interrupt");

        let (fnc, arg) = crate::ble::npl::ble_os_adapter_chip_specific::ISR_INTERRUPT_4;

        trace!("interrupt BT_MAC {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            trace!("interrupt BT_MAC call");

            let fnc: fn(*mut binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
            trace!("BT_MAC done");
        }

        trace!("interrupt BT_MAC done");
    };
}
