use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
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
    #[allow(dead_code)]
    fn table(&self) -> &'static str {
        if RELAX_MODES.contains(self) {
            return "scores_relax";
        } else if VANILLA_MODES.contains(self) {
            return "scores";
        } else {
            return "scores_ap";
        }
    }

    #[allow(dead_code)]
    fn as_vn(self) -> i32 {
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

    #[allow(dead_code)]
    fn sort(self) -> &'static str {
        if (self as i32) > 3 {
            return "pp";
        } else {
            return "score";
        }
    }

    #[allow(dead_code)]
    fn from_mods(mode: i32, mods: i32) -> Self {
        if mods & 128 > 0 { // 128 = relax
            if mode == 3 {
                return Self::mania;
            }

            return unsafe { std::mem::transmute(mode + 4) };
        } else if mods & 8192 > 0 { // 8192 = autopilot
            if mode != 0 {
                return unsafe { std::mem::transmute(mode) };
            }

            return Self::std_ap;
        }

        return unsafe { std::mem::transmute(mode) };
    }
}