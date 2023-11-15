#[allow(unused_imports)]
use self::super::root;
extern "C" {
    #[link_name = "\u{1}_ZN2nn4time10InitializeEv"]
    pub fn Initialize() -> root::Result;
    #[link_name = "\u{1}_ZN2nn4time8FinalizeEv"]
    pub fn Finalize() -> root::Result;
    #[link_name = "\u{1}_ZN2nn4time13IsInitializedEv"]
    pub fn IsInitialized() -> bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CalendarTime {
    pub year: root::s16,
    pub month: root::s8,
    pub day: root::s8,
    pub hour: root::s8,
    pub minute: root::s8,
    pub second: root::s8,
}

pub const DayOfTheWeek_Sunday: root::nn::time::DayOfTheWeek = 0;
pub const DayOfTheWeek_Monday: root::nn::time::DayOfTheWeek = 1;
pub const DayOfTheWeek_Tuesday: root::nn::time::DayOfTheWeek = 2;
pub const DayOfTheWeek_Wednesday: root::nn::time::DayOfTheWeek = 3;
pub const DayOfTheWeek_Thursday: root::nn::time::DayOfTheWeek = 4;
pub const DayOfTheWeek_Friday: root::nn::time::DayOfTheWeek = 5;
pub const DayOfTheWeek_Saturday: root::nn::time::DayOfTheWeek = 6;
pub type DayOfTheWeek = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct TimeZone {
    pub standardTimeName: [u8; 8usize],
    pub _9: bool,
    pub utcOffset: root::s32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CalendarAdditionalInfo {
    pub dayOfTheWeek: root::nn::time::DayOfTheWeek,
    pub dayofYear: root::s32,
    pub timeZone: root::nn::time::TimeZone,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PosixTime {
    pub time: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StandardUserSystemClock {
    pub _address: u8,
}

extern "C" {
    #[link_name = "\u{1}_ZN2nn4time23StandardUserSystemClock14GetCurrentTimeEPNS0_9PosixTimeE"]
    pub fn StandardUserSystemClock_GetCurrentTime(
        arg1: *mut root::nn::time::PosixTime,
    ) -> root::Result;
}
impl StandardUserSystemClock {
    #[inline]
    pub fn GetCurrentTime(arg1: *mut root::nn::time::PosixTime) -> root::Result {
        unsafe { StandardUserSystemClock_GetCurrentTime(arg1) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeZoneRule {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn4time14ToCalendarTimeEPNS0_12CalendarTimeEPNS0_22CalendarAdditionalInfoERKNS0_9PosixTimeE"]
    pub fn ToCalendarTime(
        arg1: *mut root::nn::time::CalendarTime,
        arg2: *mut root::nn::time::CalendarAdditionalInfo,
        arg3: *const root::nn::time::PosixTime,
    ) -> root::Result;
}

pub fn get_calendar_time() -> CalendarTime {
    let mut do_init = false;
    unsafe {
        if !IsInitialized() {
            do_init = true;
            Initialize();
        }
    }
    let mut calendar = CalendarTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0 };

    let mut calendar_info = CalendarAdditionalInfo { dayOfTheWeek: 0, dayofYear: 0, timeZone: TimeZone::default()};

    let mut posix_time = PosixTime { time: 0 };
    unsafe { StandardUserSystemClock_GetCurrentTime(&mut posix_time)};

    unsafe { ToCalendarTime(&mut calendar, &mut calendar_info, &posix_time)};

    if do_init {
        unsafe { Finalize(); }
    }

    return calendar;
}
