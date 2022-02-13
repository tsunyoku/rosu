use crate::db;

use crate::constants::mode::Mode;

#[derive(sqlx::FromRow)]
pub struct Stats {
    pub total_score: i32,
    pub ranked_score: i32,
    pub accuracy: f32,
    pub playcount: i32,
    pub pp: i32,
    // TODO: rank
}

impl Stats {
    pub async fn for_mode(mode: Mode, user_id: i32) -> Self {
        let query: String = format!(
            "select total_score_{suffix} as total_score, ranked_score_{suffix} as ranked_score, 
            avg_accuracy_{suffix} as accuracy, playcount_{suffix} as playcount, pp_{suffix} as pp 
            from {table} where id = ?",
            suffix = mode.sql_suffix(),
            table = mode.stats_table(),
        );

        let mode = sqlx::query_as::<_, Self>(&query)
            .bind(user_id)
            .fetch_one(db.get().unwrap())
            .await
            .unwrap();

        return mode;
    }
}