pub mod company;
pub mod portfolio;
pub mod valuation;

use serde::{Deserialize, Serialize};

/// A publicly traded company.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub ticker: String,
    pub name: String,
    pub exchange: String,
    pub sector: String,
    pub industry: String,
    pub market_cap: Option<f64>,
    pub employees: Option<i64>,
    pub description: String,
}

/// Comprehensive financial statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialStatements {
    pub ticker: String,
    pub income_statement: Vec<IncomeStatementYear>,
    pub balance_sheet: Vec<BalanceSheetYear>,
    pub cash_flow: Vec<CashFlowYear>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatementYear {
    pub fiscal_year: i32,
    pub revenue: f64,
    pub cost_of_revenue: f64,
    pub gross_profit: f64,
    pub operating_expenses: f64,
    pub operating_income: f64,
    pub interest_expense: f64,
    pub income_before_tax: f64,
    pub income_tax: f64,
    pub net_income: f64,
    pub eps_basic: f64,
    pub eps_diluted: f64,
    pub shares_outstanding: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheetYear {
    pub fiscal_year: i32,
    pub total_assets: f64,
    pub current_assets: f64,
    pub cash_and_equivalents: f64,
    pub total_liabilities: f64,
    pub current_liabilities: f64,
    pub long_term_debt: f64,
    pub total_equity: f64,
    pub retained_earnings: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowYear {
    pub fiscal_year: i32,
    pub operating_cash_flow: f64,
    pub capital_expenditure: f64,
    pub free_cash_flow: f64,
    pub dividends_paid: f64,
    pub net_change_in_debt: f64,
}
