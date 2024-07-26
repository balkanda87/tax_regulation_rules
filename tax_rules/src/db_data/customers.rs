// Author:  Balaji Kandasamy
// Email: balkanda87@outlook.com
// Maintainer: Someone
// GitHub: @balkanda87
// Licensed under the MIT License.

use sqlx::{PgPool, Row};

pub async fn fetch_customer_details(pool: &PgPool) -> Result<Vec<(i32, String, String, String, String)>, sqlx::Error> {
    let query_result = sqlx::query("SELECT * FROM customer_details")
        .fetch_all(pool)
        .await?;

    let result: Vec<(i32, String, String, String, String)> = query_result
        .iter()
        .map(|row| (
            row.get("customer_id"),
            row.get("customer_name"),
            row.get("pan_id"),
            row.get("aadhaar_number"),
            row.get("mobile_number"),
        ))
        .collect();

    Ok(result)
}