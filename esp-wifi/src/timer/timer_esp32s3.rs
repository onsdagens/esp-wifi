use crate::hal::{interrupt, macros::interrupt, peripherals};

pub fn setup_radio_isr() {
    #[cfg(feature = "wifi")]
    {
        unwrap!(interrupt::enable(
            peripherals::Interrupt::WIFI_MAC,
            interrupt::Priority::Priority1,
        ));
        unwrap!(interrupt::enable(
            peripherals::Interrupt::WIFI_PWR,
            interrupt::Priority::Priority1,
        ));
    }

    #[cfg(feature = "ble")]
    {
        unwrap!(interrupt::enable(
            peripherals::Interrupt::BT_BB,
            interrupt::Priority::Priority1,
        ));
        unwrap!(interrupt::enable(
            peripherals::Interrupt::RWBLE,
            interrupt::Priority::Priority1,
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
            let fnc: fn(*mut crate::binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }
    }
}

#[cfg(feature = "wifi")]
#[interrupt]
fn WIFI_PWR() {
    unsafe {
        let (fnc, arg) = crate::wifi::os_adapter::ISR_INTERRUPT_1;
        trace!("interrupt WIFI_PWR {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            let fnc: fn(*mut crate::binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }

        trace!("interrupt 1 done");
    };
}

#[cfg(feature = "ble")]
#[interrupt]
fn RWBLE() {
    critical_section::with(|_| unsafe {
        let (fnc, arg) = crate::ble::btdm::ble_os_adapter_chip_specific::ISR_INTERRUPT_5;
        trace!("interrupt RWBLE {:?} {:?}", fnc, arg);
        if !fnc.is_null() {
            let fnc: fn(*mut crate::binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }
    });
}

#[cfg(feature = "ble")]
#[interrupt]
fn BT_BB() {
    critical_section::with(|_| unsafe {
        let (fnc, arg) = crate::ble::btdm::ble_os_adapter_chip_specific::ISR_INTERRUPT_8;
        trace!("interrupt RWBT {:?} {:?}", fnc, arg);

        if !fnc.is_null() {
            let fnc: fn(*mut crate::binary::c_types::c_void) = core::mem::transmute(fnc);
            fnc(arg);
        }
    });
}
