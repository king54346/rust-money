mod constants;
mod currency;
mod formatter;

use std::cmp::Ordering;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt;
use crate::currency::Currency;
use crate::formatter::Formatter;


#[derive(Debug, Clone, PartialEq,Serialize)]
struct Money {
    amount: i64,
    currency: Currency,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = &self.currency;
        let formatted_amount = c.formatter().format(self.amount);
        write!(f, "{} {}", c.code, formatted_amount)
    }
}

impl Money {
    fn new(amount: i64, code: &str) -> Self {
        let mut currency = Currency::new(code);
        currency.get();
        Money {
            amount,
            currency,
        }
    }

    fn new_from_float(amount: f64, code: &str) -> Self {
        let mut currency = Currency::new(code); // 假设货币的小数位数为2，可以根据实际情况调整
        currency.get();
        let factor = 10i64.pow(currency.fraction as u32);
        let amount = (amount * factor as f64).floor() as i64;
        Money {
            amount,
            currency,
        }
    }

    fn currency(&self) -> &Currency {
        &self.currency
    }

    fn amount(&self) -> i64 {
        self.amount
    }

    fn same_currency(&self, other: &Money) -> bool {
        self.currency.eq(&other.currency)
    }

    fn assert_same_currency(&self, other: &Money) -> Result<(), MoneyError> {
        if !self.same_currency(other) {
            Err(MoneyError::CurrencyMismatch)
        } else {
            Ok(())
        }
    }

    fn compare(&self, other: &Money) -> Ordering {
        self.amount.cmp(&other.amount)
    }

    fn equals(&self, other: &Money) -> Result<bool, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.compare(other) == Ordering::Equal)
    }

    fn greater_than(&self, other: &Money) -> Result<bool, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.compare(other) == Ordering::Greater)
    }

    fn greater_than_or_equal(&self, other: &Money) -> Result<bool, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.compare(other) != Ordering::Less)
    }

    fn less_than(&self, other: &Money) -> Result<bool, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.compare(other) == Ordering::Less)
    }

    fn less_than_or_equal(&self, other: &Money) -> Result<bool, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(self.compare(other) != Ordering::Greater)
    }

    fn is_zero(&self) -> bool {
        self.amount == 0
    }
    // 正值
    fn is_positive(&self) -> bool {
        self.amount > 0
    }
    // 负值
    fn is_negative(&self) -> bool {
        self.amount < 0
    }

    fn absolute(&self) -> Self {
        Money {
            amount: self.amount.abs(),
            currency: self.currency.clone(),
        }
    }

    fn negative(&self) -> Self {
        Money {
            amount: -self.amount,
            currency: self.currency.clone(),
        }
    }

    fn add(&self, other: &Money) -> Result<Self, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    fn subtract(&self, other: &Money) -> Result<Self, MoneyError> {
        self.assert_same_currency(other)?;
        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }

    fn multiply(&self, multiplier: i64) -> Self {
        Money {
            amount: self.amount * multiplier,
            currency: self.currency.clone(),
        }
    }

    fn round(&self) -> Self {
        let factor = 10i64.pow(self.currency.fraction as u32);
        let rounded_amount = ((self.amount as f64 / factor as f64).round() * factor as f64) as i64;
        Money {
            amount: rounded_amount,
            currency: self.currency.clone(),
        }
    }

    fn split(&self, parts: usize) -> Result<Vec<Money>, MoneyError> {
        if parts <= 0 {
            return Err(MoneyError::InvalidSplit);
        }
        // 计算每部分的金额和余数
        let part_amount = self.amount / parts as i64;
        let remainder = self.amount % parts as i64;
        let mut result = vec![Money::new(part_amount, &self.currency.code); parts];
        // 将余数加到相应的金额上
        for i in 0..remainder.abs() as usize {
            result[i].amount += remainder.signum();
        }

        Ok(result)
    }

    fn allocate(&self, ratios: &[u32]) -> Result<Vec<Money>, MoneyError> {
        if ratios.is_empty() {
            return Err(MoneyError::NoRatios);
        }
        // 计算 ratios 数组的总和
        let total: u32 = ratios.iter().sum();
        if total == 0 {
            return Ok(vec![Money::new(0, &self.currency.code); ratios.len()]);
        }

        let mut result = Vec::with_capacity(ratios.len());
        // 用于跟踪已分配的总金额
        let mut total_allocated = 0;
        for &ratio in ratios {
            let amount = self.amount * ratio as i64 / total as i64;
            result.push(Money::new(amount, &self.currency.code));
            total_allocated += amount;
        }
        // 计算剩余金额（remainder），这是由于整数除法可能会产生的
        // 将 remainder.signum() 加到相应的金额上
        let remainder = self.amount - total_allocated;
        for i in 0..remainder.abs() as usize {
            result[i].amount += remainder.signum();
        }

        Ok(result)
    }

    fn compare_money(&self, other: &Money) -> Result<i32, MoneyError> {
        if let Err(err) = self.assert_same_currency(other) {
            return Err(err);
        }

        let cmp = self.compare(other);
        let result = match cmp {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        };

        Ok(result)
    }

    // 表示为给定货币值的子单位 (float64)
    fn as_major_units(&self) -> f64 {
        self.currency.formatter().to_major_units(self.amount)
    }

}

#[derive(Debug)]
enum MoneyError {
    CurrencyMismatch,
    InvalidSplit,
    NoRatios,
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoneyError::CurrencyMismatch => write!(f, "Currency mismatch"),
            MoneyError::InvalidSplit => write!(f, "Invalid split value"),
            MoneyError::NoRatios => write!(f, "No ratios specified"),
        }
    }
}

impl Error for MoneyError {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_money() {
        // Example usage:
        let m1 = Money::new(100, "USD");
        let result = m1.add(&m1).unwrap();
        let result1 = result.split(3).unwrap();
        println!("{}", result1[0]);
        println!("{}", result1[1]);
        println!("{}", result1[2]);
    }
    #[test]
    fn example_new() {
        // Example usage:
        let m1 = Money::new(100,  "GBP");
        println!("{}", m1);
    }
    #[test]
    fn example_money_comparisons() {
        // Example usage:
        let m1 = Money::new(100,  "GBP");
        let m2 = Money::new(200,  "GBP");
        let m3 = Money::new(200,  "EUR");
        let x = m1.greater_than(&m2);
        println!("{:?}", x);
        let result = m1.less_than(&m2);
        println!("{:?}", result);
        let result1 = m2.equals(&m3);
        println!("{:?}", result1);
    }
    #[test]
    fn example_money_is_negative() {
        // Example usage:
        let m1 = Money::new(100,  "GBP");
        println!("{}", m1.is_negative());
    }
    #[test]
    fn example_money_add() {
        // Example usage:
        let m1 = Money::new(100,  "GBP");
        let m2 = Money::new(200,  "GBP");
        let result = m1.add(&m2).unwrap();
        println!("{}", result);
    }
    #[test]
    fn example_money_subtract() {
        // Example usage:
        let m1 = Money::new(100,  "GBP");
        let result = m1.multiply(2);
        println!("{}", result);
    }
    #[test]
    fn example_money_absolute() {
        // Example usage:
        let m1 = Money::new(-100,  "cny");
        let result = m1.absolute();
        println!("{}", result);
    }
    #[test]
    fn example_money_allocate() {
        // Example usage:
        let m1 = Money::new(-100,  "GBP");
        let result = m1.allocate(&[33,33,33]).unwrap();
        println!("{}", result[0]);
        println!("{}", result[1]);
        println!("{}", result[2]);
    }
    #[test]
    fn example_money_as_major_units() {
        // Example usage:
        let m1 = Money::new(123456789,  "EUR").as_major_units();
        println!("{}", m1);
    }
}
