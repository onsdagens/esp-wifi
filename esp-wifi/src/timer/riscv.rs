use core::cell::RefCell;

use critical_section::Mutex;

use crate::{
    hal::{
        interrupt::{self, TrapFrame},
        peripherals::{self, Interrupt},
        prelude::*,
        riscv,
        systimer::{Alarm, Periodic, SystemTimer, Target},
    },
    preempt::preempt::task_switch,
};

#[cfg(feature = "esp32c6")]
use peripherals::INTPRI as SystemPeripheral;
#[cfg(not(feature = "esp32c6"))]
use peripherals::SYSTEM as SystemPeripheral;

pub type TimeBase = Alarm<Target, 0>;

pub const TICKS_PER_SECOND: u64 = 16_000_000;

const TIMER_DELAY: fugit::HertzU32 = fugit::HertzU32::from_raw(crate::CONFIG.tick_rate_hz);

static ALARM0: Mutex<RefCell<Option<Alarm<Periodic, 0>>>> = Mutex::new(RefCell::new(None));

pub fn setup_timer(systimer: TimeBase) {
    let alarm0 = systimer.into_periodic();
    alarm0.set_period(TIMER_DELAY.into());
    alarm0.clear_interrupt();
    alarm0.interrupt_enable(true);

    critical_section::with(|cs| ALARM0.borrow_ref_mut(cs).replace(alarm0));

    unwrap!(unsafe{interrupt::enable(
        Interrupt::SYSTIMER_TARGET0,
        interrupt::Priority::Priority15,
        interrupt::CpuInterrupt::Interrupt28,
    )});
}

pub fn setup_multitasking() {
    unwrap!(unsafe{interrupt::enable(
        Interrupt::FROM_CPU_INTR3,
        interrupt::Priority::Priority15,
        interrupt::CpuInterrupt::Interrupt27,
    )});

    unsafe {
        riscv::interrupt::enable();
    }

    while unsafe { crate::preempt::FIRST_SWITCH.load(core::sync::atomic::Ordering::Relaxed) } {}
}

#[export_name="cpu_int_28_handler"]
fn SYSTIMER_TARGET0(trap_frame: &mut TrapFrame) {
    // clear the systimer intr
    critical_section::with(|cs| {
        unwrap!(ALARM0.borrow_ref_mut(cs).as_mut()).clear_interrupt();
    });

    task_switch(trap_frame);
}

#[export_name="cpu_int_27_handler"]
fn FROM_CPU_INTR3(trap_frame: &mut TrapFrame) {
    unsafe {
        // clear FROM_CPU_INTR3
        (&*SystemPeripheral::PTR)
            .cpu_intr_from_cpu_3
            .modify(|_, w| w.cpu_intr_from_cpu_3().clear_bit());
    }

    critical_section::with(|cs| {
        let mut alarm0 = ALARM0.borrow_ref_mut(cs);
        let alarm0 = unwrap!(alarm0.as_mut());

        alarm0.set_period(TIMER_DELAY.into());
        alarm0.clear_interrupt();
    });

    task_switch(trap_frame);
}

pub fn yield_task() {
    unsafe {
        (&*SystemPeripheral::PTR)
            .cpu_intr_from_cpu_3
            .modify(|_, w| w.cpu_intr_from_cpu_3().set_bit());
    }
}

/// Current systimer count value
/// A tick is 1 / 16_000_000 seconds
pub fn get_systimer_count() -> u64 {
    SystemTimer::now()
}
