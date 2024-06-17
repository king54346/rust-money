use std::cmp;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Formatter {
    pub fraction: usize,
    pub decimal: String,
    pub thousand: String,
    pub grapheme: String,
    pub template: String,
}

impl Formatter {
    pub fn new(fraction: usize, decimal: &str, thousand: &str, grapheme: &str, template: &str) -> Self {
        Self {
            fraction,
            decimal: decimal.to_string(),
            thousand: thousand.to_string(),
            grapheme: grapheme.to_string(),
            template: template.to_string(),
        }
    }

    pub fn format(&self, amount: i64) -> String {
        // Work with absolute amount value
        let mut sa = self.abs(amount).to_string();

        if sa.len() <= self.fraction {
            sa = "0".repeat(self.fraction - sa.len() + 1) + &sa;
        }

        if !self.thousand.is_empty() {
            let mut i = sa.len() as isize - self.fraction as isize - 3;
            while i > 0 {
                let i_usize = i as usize;
                sa.insert_str(i_usize, &self.thousand);
                i -= 3;
            }
        }

        if self.fraction > 0 {
            let len = sa.len();
            sa.insert(len - self.fraction, self.decimal.chars().next().unwrap());
        }

        let mut result = self.template.replace('1', &sa);
        result = result.replace('$', &self.grapheme);

        // Add minus sign for negative amount.
        if amount < 0 {
            result = "-".to_string() + &result;
        }

        result
    }

    pub fn to_major_units(&self, amount: i64) -> f64 {
        if self.fraction == 0 {
            return amount as f64;
        }

        amount as f64 / 10_f64.powi(self.fraction as i32)
    }

    fn abs(&self, amount: i64) -> i64 {
        cmp::max(amount, -amount)
    }
}