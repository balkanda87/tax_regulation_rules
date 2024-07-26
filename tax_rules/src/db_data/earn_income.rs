// Author:  Balaji Kandasamy
// Email: balkanda87@outlook.com
// Maintainer: Someone
// GitHub: @balkanda87
// Licensed under the MIT License.


use sqlx::{PgPool, Row};

pub async fn fetch_earned_income(pool: &PgPool) -> Result<Vec<(i32, String, i32, String, String, f64, f64, i32)>, sqlx::Error> {
    let query_result = sqlx::query("SELECT * FROM earned_income")
        .fetch_all(pool)
        .await?;

    let result: Vec<(i32, String, i32, String, String, f64, f64, i32)> = query_result
        .iter()
        .map(|row| (
            row.get(0),
            row.get(1),
            row.get(2),
            row.get(3),
            row.get(4),
            row.get(5),
            row.get(6),
            row.get(7),
        ))
        .collect();

    Ok(result)
}
