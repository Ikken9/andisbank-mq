use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Loan {
    pub id: u32,
    pub user_id: u32,
    pub loan_type_id: u32,
    pub amount: f32,
    pub currency: String,
    pub term_months: String,
    pub interest_rate: f32,
    pub monthly_payment: f32,
    pub balance: f32,
    pub status: String,
    pub start_date: String,
    pub end_date: String
}

impl Display for Loan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \n{} \n{} \n{} \n{} \n{} \n{} \n{} \n{} \n{} \n{} \n{}",
               self.id,
               self.user_id,
               self.loan_type_id,
               self.amount,
               self.currency,
               self.term_months,
               self.interest_rate,
               self.monthly_payment,
               self.balance,
               self.status,
               self.start_date,
               self.end_date
        )
    }
}