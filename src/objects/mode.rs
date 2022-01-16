use ntex::web;
use num_enum::TryFromPrimitive;
use sqlx::{MySql, Pool};

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
    fn stats_table(&self) -> &'static str {
        if RELAX_MODES.contains(self) {
            return "rx_stats";
        } else if VANILLA_MODES.contains(self) {
            return "users_stats";
        } else {
            return "ap_stats";
        }
    }

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

    fn sql_suffix(self) -> &'static str {
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

    fn from_mods(mode: i32, mods: i32) -> Self {
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

#[derive(sqlx::FromRow)]
pub struct Stats {
    pub total_score: i32,
    pub ranked_score: i32,
    pub accuracy: f32,
    pub playcount: i32,
    pub pp: i32,
    // TODO: rank
}

type DBPool = web::types::Data<Pool<MySql>>;

impl Stats {
    pub async fn for_mode(mode: Mode, user_id: i32, pool: &DBPool) -> Self {
        let query: String = format!(
            "select total_score_{suffix} as total_score, ranked_score_{suffix} as ranked_score, 
            avg_accuracy_{suffix} as accuracy, playcount_{suffix} as playcount, pp_{suffix} as pp 
            from {table} where id = ?",
            suffix = mode.sql_suffix(),
            table = mode.stats_table(),
        );

        let mode = sqlx::query_as::<_, Self>(&query)
            .bind(user_id)
            .fetch_one(&***pool)
            .await
            .unwrap();

        return mode;
    }
}
