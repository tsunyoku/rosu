use num_enum::TryFromPrimitive;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive, EnumIter)]
#[repr(i32)]
pub enum Mode {
    std = 0,
    taiko = 1,
    catch = 2,
    mania = 3,

    std_rx = 4,
    taiko_rx = 5,
    catch_rx = 6,
    std_ap = 7,
}

// these are ugly but sadly enums in rust aren't as good as python/c++ :|
const STD_MODES: &[Mode; 3] = &[Mode::std, Mode::std_rx, Mode::std_ap];
const TAIKO_MODES: &[Mode; 2] = &[Mode::taiko, Mode::taiko_rx];
const CATCH_MODES: &[Mode; 2] = &[Mode::catch, Mode::catch_rx];

const RELAX_MODES: &[Mode; 3] = &[Mode::std_rx, Mode::taiko_rx, Mode::catch_rx];
const VANILLA_MODES: &[Mode; 4] = &[Mode::std, Mode::taiko, Mode::catch, Mode::mania];

impl Mode {
    pub fn stats_table(&self) -> &'static str {
        if RELAX_MODES.contains(self) {
            return "rx_stats";
        } else if VANILLA_MODES.contains(self) {
            return "users_stats";
        } else {
            return "ap_stats";
        }
    }

    pub fn as_vn(self) -> i32 {
        if STD_MODES.contains(&self) {
            return 0;
        } else if TAIKO_MODES.contains(&self) {
            return 1;
        } else if CATCH_MODES.contains(&self) {
            return 2;
        } else {
            return self as i32;
        }
    }

    pub fn sql_suffix(self) -> &'static str {
        if STD_MODES.contains(&self) {
            return "std";
        } else if TAIKO_MODES.contains(&self) {
            return "taiko";
        } else if CATCH_MODES.contains(&self) {
            return "ctb";
        } else {
            return "mania";
        }
    }

    pub fn from_mods(mode: i32, mods: i32) -> Self {
        if mods & 128 > 0 {
            // 128 = relax
            if mode == 3 {
                return Self::mania;
            }

            return unsafe { std::mem::transmute(mode + 4) };
        } else if mods & 8192 > 0 {
            // 8192 = autopilot
            if mode != 0 {
                return unsafe { std::mem::transmute(mode) };
            }

            return Self::std_ap;
        }

        return unsafe { std::mem::transmute(mode) };
    }
}
