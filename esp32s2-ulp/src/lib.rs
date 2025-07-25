#![doc = "Peripheral access API for ESP32-S2-ULP microcontrollers (generated using svd2rust v0.36.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/46717278")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 0;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn TOUCH_DONE_INT();
    fn TOUCH_INACTIVE_INT();
    fn TOUCH_ACTIVE_INT();
    fn SARADC1_DONE_INT();
    fn SARADC2_DONE_INT();
    fn TSENS_DONE_INT();
    fn RISCV_START_INT();
    fn SW_INT();
    fn SWD_INT();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    pub _reserved: usize,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".rwtext"]
#[no_mangle]
pub static __EXTERNAL_INTERRUPTS: [Vector; 9] = [
    Vector {
        _handler: TOUCH_DONE_INT,
    },
    Vector {
        _handler: TOUCH_INACTIVE_INT,
    },
    Vector {
        _handler: TOUCH_ACTIVE_INT,
    },
    Vector {
        _handler: SARADC1_DONE_INT,
    },
    Vector {
        _handler: SARADC2_DONE_INT,
    },
    Vector {
        _handler: TSENS_DONE_INT,
    },
    Vector {
        _handler: RISCV_START_INT,
    },
    Vector { _handler: SW_INT },
    Vector { _handler: SWD_INT },
];
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "Low-power Input/Output"]
pub type RTC_IO = crate::Periph<rtc_io::RegisterBlock, 0xa400>;
impl core::fmt::Debug for RTC_IO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_IO").finish()
    }
}
#[doc = "Low-power Input/Output"]
pub mod rtc_io;
#[doc = "Real-Time Clock Control"]
pub type RTC_CNTL = crate::Periph<rtc_cntl::RegisterBlock, 0x8000>;
impl core::fmt::Debug for RTC_CNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL").finish()
    }
}
#[doc = "Real-Time Clock Control"]
pub mod rtc_cntl;
#[doc = "Low-power I2C (Inter-Integrated Circuit) Controller"]
pub type RTC_I2C = crate::Periph<rtc_i2c::RegisterBlock, 0xec00>;
impl core::fmt::Debug for RTC_I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_I2C").finish()
    }
}
#[doc = "Low-power I2C (Inter-Integrated Circuit) Controller"]
pub mod rtc_i2c;
#[doc = "SENS Peripheral"]
pub type SENS = crate::Periph<sens::RegisterBlock, 0xc800>;
impl core::fmt::Debug for SENS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENS").finish()
    }
}
#[doc = "SENS Peripheral"]
pub mod sens;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "RTC_IO"]
    pub RTC_IO: RTC_IO,
    #[doc = "RTC_CNTL"]
    pub RTC_CNTL: RTC_CNTL,
    #[doc = "RTC_I2C"]
    pub RTC_I2C: RTC_I2C,
    #[doc = "SENS"]
    pub SENS: SENS,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            RTC_IO: RTC_IO::steal(),
            RTC_CNTL: RTC_CNTL::steal(),
            RTC_I2C: RTC_I2C::steal(),
            SENS: SENS::steal(),
        }
    }
}
