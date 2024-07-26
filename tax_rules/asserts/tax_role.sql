-- Create the tax_roles database
--CREATE DATABASE tax_roles;

-- Create the tax_role user (replace 'your_password' with an actual password)
--CREATE USER tax_role WITH PASSWORD 'testuser';

-- Grant necessary privileges to the tax_role user
--GRANT ALL PRIVILEGES ON DATABASE tax_roles TO tax_role;

-- switch to tax_role db
--\connect  tax_role

-- Drop table if already exist
DROP TABLE IF EXISTS customer_details CASCADE;
DROP TABLE IF EXISTS income_rules;
DROP TABLE IF EXISTS earned_income;
DROP TABLE IF EXISTS passive_income;
DROP TABLE IF EXISTS portfolio_income;


-- Create Customer details table 
CREATE TABLE customer_details (
    customer_id SERIAL PRIMARY KEY,
    customer_name VARCHAR(255) NOT NULL,
    pan_id VARCHAR(12) UNIQUE NOT NULL,
	aadhaar_number VARCHAR(18) UNIQUE NOT NULL,
	mobile_number VARCHAR(15) UNIQUE NOT NULL
);

-- Create Income rules table 
CREATE TABLE income_rules (
    id SERIAL PRIMARY KEY,
	pan_id VARCHAR(12) REFERENCES customer_details(pan_id),
    rule_type VARCHAR(50),
    rule_data JSONB
);

-- Create earned income table 
CREATE TABLE earned_income (
    income_id SERIAL PRIMARY KEY,
    pan_id VARCHAR(12) REFERENCES customer_details(pan_id),
    emp_id INT UNIQUE,
    emp_name VARCHAR(255),
	for_month VARCHAR(100),
    salary DECIMAL(10, 2),
    commission DECIMAL(10, 2),
    dept_id INT
);

-- Create passive income table 
CREATE TABLE passive_income (
    income_id SERIAL PRIMARY KEY,
    pan_id VARCHAR(12) REFERENCES customer_details(pan_id),
    property_id INT UNIQUE,
    property_name VARCHAR(255),
    monthly_rent DECIMAL(10, 2),
    location VARCHAR(255)
);

-- Create portfolio income table 
CREATE TABLE portfolio_income (
    income_id SERIAL PRIMARY KEY,
    pan_id VARCHAR(12) REFERENCES customer_details(pan_id),
	bank_account VARCHAR(50),
	bank_name VARCHAR(255),
	ifsc_code VARCHAR(100),
	bank_interest VARCHAR(255),
    stock_id INT UNIQUE,
    stock_symbol VARCHAR(12),
    dividend_yield DECIMAL(5, 2),
    sector VARCHAR(50)
);

COPY customer_details(customer_name, pan_id, aadhaar_number, mobile_number)
FROM 'C:\Users\Balaji Kandasamy\Documents\CustomerProjects\tax_rules\tax_regulation_rules\tax_rules\asserts\customer_details.csv'
DELIMITER ',' CSV HEADER;

COPY earned_income(pan_id, emp_id, emp_name, for_month, salary, commission, dept_id)
FROM 'C:\Users\Balaji Kandasamy\Documents\CustomerProjects\tax_rules\tax_regulation_rules\tax_rules\asserts\earned_income.csv'
DELIMITER ',' CSV HEADER;

COPY passive_income(pan_id, property_id, property_name, monthly_rent, location)
FROM 'C:\Users\Balaji Kandasamy\Documents\CustomerProjects\tax_rules\tax_regulation_rules\tax_rules\asserts\customer_details.csv'
DELIMITER ',' CSV HEADER;

COPY portfolio_income(pan_id, bank_account, bank_name, ifsc_code, bank_interest, stock_id, stock_symbol, dividend_yield, sector)
FROM 'C:\Users\Balaji Kandasamy\Documents\CustomerProjects\tax_rules\tax_regulation_rules\tax_rules\asserts\customer_details.csv'
DELIMITER ',' CSV HEADER;