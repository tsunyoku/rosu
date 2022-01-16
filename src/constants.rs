use serde::{Serialize, Serializer};
use strum_macros::EnumString;
use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(i16)]
pub enum Packets {
    OSU_CHANGE_ACTION = 0,
    OSU_SEND_PUBLIC_MESSAGE = 1,
    OSU_LOGOUT = 2,
    OSU_REQUEST_STATUS_UPDATE = 3,
    OSU_PING = 4,
    CHO_USER_ID = 5,
    CHO_SEND_MESSAGE = 7,
    CHO_PONG = 8,
    CHO_HANDLE_IRC_CHANGE_USERNAME = 9,
    CHO_HANDLE_IRC_QUIT = 10,
    CHO_USER_STATS = 11,
    CHO_USER_LOGOUT = 12,
    CHO_SPECTATOR_JOINED = 13,
    CHO_SPECTATOR_LEFT = 14,
    CHO_SPECTATE_FRAMES = 15,
    OSU_START_SPECTATING = 16,
    OSU_STOP_SPECTATING = 17,
    OSU_SPECTATE_FRAMES = 18,
    CHO_VERSION_UPDATE = 19,
    OSU_ERROR_REPORT = 20,
    OSU_CANT_SPECTATE = 21,
    CHO_SPECTATOR_CANT_SPECTATE = 22,
    CHO_GET_ATTENTION = 23,
    CHO_NOTIFICATION = 24,
    OSU_SEND_PRIVATE_MESSAGE = 25,
    CHO_UPDATE_MATCH = 26,
    CHO_NEW_MATCH = 27,
    CHO_DISPOSE_MATCH = 28,
    OSU_PART_LOBBY = 29,
    OSU_JOIN_LOBBY = 30,
    OSU_CREATE_MATCH = 31,
    OSU_JOIN_MATCH = 32,
    OSU_PART_MATCH = 33,
    CHO_TOGGLE_BLOCK_NON_FRIEND_DMS = 34,
    CHO_MATCH_JOIN_SUCCESS = 36,
    CHO_MATCH_JOIN_FAIL = 37,
    OSU_MATCH_CHANGE_SLOT = 38,
    OSU_MATCH_READY = 39,
    OSU_MATCH_LOCK = 40,
    OSU_MATCH_CHANGE_SETTINGS = 41,
    CHO_FELLOW_SPECTATOR_JOINED = 42,
    CHO_FELLOW_SPECTATOR_LEFT = 43,
    OSU_MATCH_START = 44,
    CHO_ALL_PLAYERS_LOADED = 45,
    CHO_MATCH_START = 46,
    OSU_MATCH_SCORE_UPDATE = 47,
    CHO_MATCH_SCORE_UPDATE = 48,
    OSU_MATCH_COMPLETE = 49,
    CHO_MATCH_TRANSFER_HOST = 50,
    OSU_MATCH_CHANGE_MODS = 51,
    OSU_MATCH_LOAD_COMPLETE = 52,
    CHO_MATCH_ALL_PLAYERS_LOADED = 53,
    OSU_MATCH_NO_BEATMAP = 54,
    OSU_MATCH_NOT_READY = 55,
    OSU_MATCH_FAILED = 56,
    CHO_MATCH_PLAYER_FAILED = 57,
    CHO_MATCH_COMPLETE = 58,
    OSU_MATCH_HAS_BEATMAP = 59,
    OSU_MATCH_SKIP_REQUEST = 60,
    CHO_MATCH_SKIP = 61,
    CHO_UNAUTHORIZED = 62, // unused
    OSU_CHANNEL_JOIN = 63,
    CHO_CHANNEL_JOIN_SUCCESS = 64,
    CHO_CHANNEL_INFO = 65,
    CHO_CHANNEL_KICK = 66,
    CHO_CHANNEL_AUTO_JOIN = 67,
    OSU_BEATMAP_INFO_REQUEST = 68,
    CHO_BEATMAP_INFO_REPLY = 69,
    OSU_MATCH_TRANSFER_HOST = 70,
    CHO_PRIVILEGES = 71,
    CHO_FRIENDS_LIST = 72,
    OSU_FRIEND_ADD = 73,
    OSU_FRIEND_REMOVE = 74,
    CHO_PROTOCOL_VERSION = 75,
    CHO_MAIN_MENU_ICON = 76,
    OSU_MATCH_CHANGE_TEAM = 77,
    OSU_CHANNEL_PART = 78,
    OSU_RECEIVE_UPDATES = 79,
    CHO_MONITOR = 80, // unused
    CHO_MATCH_PLAYER_SKIPPED = 81,
    OSU_SET_AWAY_MESSAGE = 82,
    CHO_USER_PRESENCE = 83,
    OSU_IRC_ONLY = 84,
    OSU_USER_STATS_REQUEST = 85,
    CHO_RESTART = 86,
    OSU_MATCH_INVITE = 87,
    CHO_MATCH_INVITE = 88,
    CHO_CHANNEL_INFO_END = 89,
    OSU_MATCH_CHANGE_PASSWORD = 90,
    CHO_MATCH_CHANGE_PASSWORD = 91,
    CHO_SILENCE_END = 92,
    OSU_TOURNAMENT_MATCH_INFO_REQUEST = 93,
    CHO_USER_SILENCED = 94,
    CHO_USER_PRESENCE_SINGLE = 95,
    CHO_USER_PRESENCE_BUNDLE = 96,
    OSU_USER_PRESENCE_REQUEST = 97,
    OSU_USER_PRESENCE_REQUEST_ALL = 98,
    OSU_TOGGLE_BLOCK_NON_FRIEND_DMS = 99,
    CHO_USER_DM_BLOCKED = 100,
    CHO_TARGET_IS_SILENCED = 101,
    CHO_VERSION_UPDATE_FORCED = 102,
    CHO_SWITCH_SERVER = 103,
    CHO_ACCOUNT_RESTRICTED = 104,
    CHO_RTX = 105, // unused
    CHO_MATCH_ABORT = 106,
    CHO_SWITCH_TOURNAMENT_SERVER = 107,
    OSU_TOURNAMENT_JOIN_MATCH_CHANNEL = 108,
    OSU_TOURNAMENT_LEAVE_MATCH_CHANNEL = 109,
}

impl Serialize for Packets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i16(*self as i16)
    }
}

#[derive(Debug, PartialEq, EnumString)]
pub enum CountryCodes {
    UN = 0,
    OC = 1,
    EU = 2,
    AD = 3,
    AE = 4,
    AF = 5,
    AG = 6,
    AI = 7,
    AL = 8,
    AM = 9,
    AN = 10,

    AO = 11,
    AQ = 12,
    AR = 13,
    AS = 14,
    AT = 15,
    AU = 16,
    AW = 17,
    AZ = 18,
    BA = 19,
    BB = 20,

    BD = 21,
    BE = 22,
    BF = 23,
    BG = 24,
    BH = 25,
    BI = 26,
    BJ = 27,
    BM = 28,
    BN = 29,
    BO = 30,

    BR = 31,
    BS = 32,
    BT = 33,
    BV = 34,
    BW = 35,
    BY = 36,
    BZ = 37,
    CA = 38,
    CC = 39,
    CD = 40,

    CF = 41,
    CG = 42,
    CH = 43,
    CI = 44,
    CK = 45,
    CL = 46,
    CM = 47,
    CN = 48,
    CO = 49,
    CR = 50,

    CU = 51,
    CV = 52,
    CX = 53,
    CY = 54,
    CZ = 55,
    DE = 56,
    DJ = 57,
    DK = 58,
    DM = 59,
    DO = 60,

    DZ = 61,
    EC = 62,
    EE = 63,
    EG = 64,
    EH = 65,
    ER = 66,
    ES = 67,
    ET = 68,
    FI = 69,
    FJ = 70,

    FK = 71,
    FM = 72,
    FO = 73,
    FR = 74,
    FX = 75,
    GA = 76,
    GB = 77,
    GD = 78,
    GE = 79,
    GF = 80,

    GH = 81,
    GI = 82,
    GL = 83,
    GM = 84,
    GN = 85,
    GP = 86,
    GQ = 87,
    GR = 88,
    GS = 89,
    GT = 90,

    GU = 91,
    GW = 92,
    GY = 93,
    HK = 94,
    HM = 95,
    HN = 96,
    HR = 97,
    HT = 98,
    HU = 99,
    ID = 100,

    IE = 101,
    IL = 102,
    IN = 103,
    IO = 104,
    IQ = 105,
    IR = 106,
    IS = 107,
    IT = 108,
    JM = 109,
    JO = 110,

    JP = 111,
    KE = 112,
    KG = 113,
    KH = 114,
    KI = 115,
    KM = 116,
    KN = 117,
    KP = 118,
    KR = 119,
    KW = 120,

    KY = 121,
    KZ = 122,
    LA = 123,
    LB = 124,
    LC = 125,
    LI = 126,
    LK = 127,
    LR = 128,
    LS = 129,
    LT = 130,

    LU = 131,
    LV = 132,
    LY = 133,
    MA = 134,
    MC = 135,
    MD = 136,
    MG = 137,
    MH = 138,
    MK = 139,
    ML = 140,

    MM = 141,
    MN = 142,
    MO = 143,
    MP = 144,
    MQ = 145,
    MR = 146,
    MS = 147,
    MT = 148,
    MU = 149,
    MV = 150,

    MW = 151,
    MX = 152,
    MY = 153,
    MZ = 154,
    NA = 155,
    NC = 156,
    NE = 157,
    NF = 158,
    NG = 159,
    NI = 160,

    NL = 161,
    NO = 162,
    NP = 163,
    NR = 164,
    NU = 165,
    NZ = 166,
    OM = 167,
    PA = 168,
    PE = 169,
    PF = 170,

    PG = 171,
    PH = 172,
    PK = 173,
    PL = 174,
    PM = 175,
    PN = 176,
    PR = 177,
    PS = 178,
    PT = 179,
    PW = 180,

    PY = 181,
    QA = 182,
    RE = 183,
    RO = 184,
    RU = 185,
    RW = 186,
    SA = 187,
    SB = 188,
    SC = 189,
    SD = 190,

    SE = 191,
    SG = 192,
    SH = 193,
    SI = 194,
    SJ = 195,
    SK = 196,
    SL = 197,
    SM = 198,
    SN = 199,
    SO = 200,

    SR = 201,
    ST = 202,
    SV = 203,
    SY = 204,
    SZ = 205,
    TC = 206,
    TD = 207,
    TF = 208,
    TG = 209,
    TH = 210,

    TJ = 211,
    TK = 212,
    TM = 213,
    TN = 214,
    TO = 215,
    TL = 216,
    TR = 217,
    TT = 218,
    TV = 219,
    TW = 220,

    TZ = 221,
    UA = 222,
    UG = 223,
    UM = 224,
    US = 225,
    UY = 226,
    UZ = 227,
    VA = 228,
    VC = 229,
    VE = 230,

    VG = 231,
    VI = 232,
    VN = 233,
    VU = 234,
    WF = 235,
    WS = 236,
    YE = 237,
    YT = 238,
    RS = 239,
    ZA = 240,

    ZM = 241,
    ME = 242,
    ZW = 243,
    XX = 244,
    A2 = 245,
    O1 = 246,
    AX = 247,
    GG = 248,
    IM = 249,
    JE = 250,

    BL = 251,
    MF = 252,
}
