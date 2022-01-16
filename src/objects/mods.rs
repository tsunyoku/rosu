use bitflags::bitflags;

bitflags! {
    pub struct Mods: i32 {
        const NOMOD = 0;
        const NOFAIL = 1 << 0;
        const EASY = 1 << 1;
        const TOUCHSCREEN = 1 << 2;
        const HIDDEN = 1 << 3;
        const HARDROCK = 1 << 4;
        const SUDDENDEATH = 1 << 5;
        const DOUBLETIME = 1 << 6;
        const RELAX = 1 << 7;
        const HALFTIME = 1 << 8;
        const NIGHTCORE = 1 << 9;
        const FLASHLIGHT = 1 << 10;
        const AUTOPLAY = 1 << 11;
        const SPUNOUT = 1 << 12;
        const AUTOPILOT = 1 << 13;
        const PERFECT = 1 << 14;
        const KEY4 = 1 << 15;
        const KEY5 = 1 << 16;
        const KEY6 = 1 << 17;
        const KEY7 = 1 << 18;
        const KEY8 = 1 << 19;
        const FADEIN = 1 << 20;
        const RANDOM = 1 << 21;
        const CINEMA = 1 << 22;
        const TARGET = 1 << 23;
        const KEY9 = 1 << 24;
        const KEYCOOP = 1 << 25;
        const KEY1 = 1 << 26;
        const KEY3 = 1 << 27;
        const KEY2 = 1 << 28;
        const SCOREV2 = 1 << 29;
        const MIRROR = 1 << 30;
    }
}

impl Mods {
    pub fn from_value(value: i32) -> Self {
        return Self { bits: value };
    }

    pub fn value(self) -> i32 {
        return self.bits();
    }
}