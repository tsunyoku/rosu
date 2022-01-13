use bitflags::bitflags;

bitflags! {
    pub struct Privileges: i64 {
        const USER_PUBLIC               = 1;
        const USER_NORMAL               = 2 << 0;
        const USER_DONOR                = 2 << 1;
        const ADMIN_ACCESS_RAP          = 2 << 2;
        const ADMIN_MANAGE_USERS        = 2 << 3;
        const ADMIN_BAN_USERS           = 2 << 4;
        const ADMIN_SILENCE_USERS       = 2 << 5;
        const ADMIN_WIPE_USERS          = 2 << 6;
        const ADMIN_MANAGE_BEATMAPS     = 2 << 7;
        const ADMIN_MANAGE_SERVERS      = 2 << 8;
        const ADMIN_MANAGE_SETTINGS     = 2 << 9;
        const ADMIN_MANAGE_BETAKEYS     = 2 << 10;
        const ADMIN_MANAGE_REPORTS      = 2 << 11;
        const ADMIN_MANAGE_DOCS         = 2 << 12;
        const ADMIN_MANAGE_BADGES       = 2 << 13;
        const ADMIN_VIEW_RAP_LOGS       = 2 << 14;
        const ADMIN_MANAGE_PRIVILEGES   = 2 << 15;
        const ADMIN_SEND_ALERTS         = 2 << 16;
        const ADMIN_CHAT_MOD            = 2 << 17;
        const ADMIN_KICK_USERS          = 2 << 18;
        const USER_PENDING_VERIFICATION = 2 << 19;
        const USER_TOURNAMENT_STAFF     = 2 << 20;
        const ADMIN_CAKER               = 20 << 21;
    }
}

impl Privileges {
    pub fn from_value(value: i64) -> Self {
        return Self { bits: value };
    }

    pub fn value(self) -> i64 {
        return self.bits();
    }
}

bitflags! {
    pub struct BanchoPrivileges: u8 {
        const PLAYER = 1 << 0;
        const MODERATOR = 1 << 1;
        const SUPPORTER = 1 << 2;
        const OWNER = 1 << 3;
        const DEVELOPER = 1 << 4;
    }
}