// Author:  Balaji Kandasamy
// Email: balkanda87@outlook.com
// Maintainer: Someone
// GitHub: @balkanda87
// Licensed under the MIT License.

use sqlx::{PgPool, Row};

pub async fn fetch_portfolio_income(pool: &PgPool) -> Result<Vec<(i32, String, String, String, String, i32, String, f64, String)>, sqlx::Error> {
    let query_result = sqlx::query("SELECT * FROM portfolio_income")
        .fetch_all(pool)
        .await?;

    let result: Vec<(i32, String, String, String, String, i32, String, f64, String)> = query_result
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
            row.get(8),
        ))
        .collect();

    Ok(result)
}