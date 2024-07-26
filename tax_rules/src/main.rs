// Author:  Balaji Kandasamy
// Email: balkanda87@outlook.com
// Maintainer: Someone
// GitHub: @balkanda87
// Licensed under the MIT License.

mod db_data;

use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use crate::db_data::connection::fetch_customer_data;

fn main() {
    // Define your tax rules as Rust data (you can customize this)
    let result = fetch_customer_data();

    match result {
        Ok(()) => println!("fetched customer details successfully!"),
        Err(e) => println!("Error: {e}"),
    }
    
    let tax_rules: Value = json!({
        "old_regime": {
            "slabs": [
                { "range": "income < 300000", "rate": 0 },
                { "range": "300000 <= income < 600000", "rate": 5 },
                { "range": "600000 <= income < 900000", "rate": 10 },
                { "range": "900000 <= income < 1200000", "rate": 15 },
                { "range": "1200000 <= income < 1500000", "rate": 25 },
                { "range": "income > 1500000", "rate": 30 },
            ],
            "deductions": [
                { "name": "Standard Deduction", "amount": 75000 },
            ],
            "rebates": [
                { "name": "Tax Rebate", "amount": 12500,
                    "slabs": [
                        { "range": "income <= 500000" },
                    ] 
                },
            ]
        },
        "new_regime": {
            "slabs": [
                { "range": "income < 300000", "rate": 0 },
                { "range": "300000 <= income < 700000", "rate": 5 },
                { "range": "700000 <= income < 1000000", "rate": 10 },
                { "range": "1000000 <= income < 1200000", "rate": 15 },
                { "range": "1200000 <= income < 1500000", "rate": 20 },
                { "range": "income > 1500000", "rate": 30 },
            ],
            "deductions": [
                { "name": "Standard Deduction", "amount": 75000 },
                { "name": "Pension Deduction", "amount": 25000 },
            ],
            "rebates": [
                { "name": "Tax Rebate", "amount": 25000,
                    "slabs": [
                        { "range": "income <= 700000" },
                    ]  
                },
            ]
        }
    });

    // Serialize to a raw JSON string
    let json_str = tax_rules.to_string();
    println!("Raw JSON:\n{}", json_str);

    // Write to a file
    if let Ok(mut file) = File::create("tax_slabs.json") {
    if let Err(err) = file.write_all(json_str.as_bytes()) {
        eprintln!("Error writing to file: {}", err);
    } else {
        println!("JSON rules written to tax_slabs.json");
    }
    } else {
        eprintln!("Error creating file.");
    }
}
