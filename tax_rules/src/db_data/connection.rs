// Author:  Balaji Kandasamy
// Email: balkanda87@outlook.com
// Maintainer: Someone
// GitHub: @balkanda87
// Licensed under the MIT License.

use sqlx::postgres::PgPoolOptions;
use crate::db_data::customers::fetch_customer_details;
use crate::db_data::earn_income::fetch_earned_income;
use crate::db_data::passive_income::fetch_passive_income;
use crate::db_data::portfolio_income::fetch_portfolio_income;

#[tokio::main]
pub async fn fetch_customer_data() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:postgres@localhost/postgres")
        .await?;

    // Fetch data from the customer_details table
    let customer_details = fetch_customer_details(&pool).await?;
    println!("Earned Income: {:?}", customer_details);

    // Fetch data from the earned_income table
    let earned_income = fetch_earned_income(&pool).await?;
    println!("Earned Income: {:?}", earned_income);

    // Fetch data from the passive_income table
    let passive_income = fetch_passive_income(&pool).await?;
    println!("Passive Income: {:?}", passive_income);

    // Fetch data from the portfolio_income table
    let portfolio_income = fetch_portfolio_income(&pool).await?;
    println!("Portfolio Income: {:?}", portfolio_income);

    //let result = vec![customer_details, earned_income, passive_income, portfolio_income];

    Ok(())
}
