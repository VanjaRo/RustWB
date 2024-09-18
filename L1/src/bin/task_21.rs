use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigNumber {
    digits: Vec<u8>, // Least-significant digit at index 0
    negative: bool,
}

impl BigNumber {
    // Create a new BigNumber from a string representation
    pub fn from_str(num: &str) -> Result<Self, String> {
        let mut digits = Vec::new();
        let mut negative = false;
        let num = num.trim();

        if num.starts_with('-') {
            negative = true;
        }

        for c in num.chars().rev() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap() as u8);
            } else if c != '-' {
                return Err("Invalid number format".to_string());
            }
        }

        Ok(BigNumber { digits, negative })
    }

    pub fn to_string(&self) -> String {
        // If no cached string, compute it
        let mut result: String = self.digits.iter().rev().map(|d| d.to_string()).collect();

        // Add the negative sign if necessary
        if self.negative {
            result.insert(0, '-');
        }

        result
    }

    pub fn add(&self, other: &BigNumber) -> BigNumber {
        if self.negative == other.negative {
            let sum_digits = self.add_digits(&other.digits);
            BigNumber {
                digits: sum_digits,
                negative: self.negative,
            }
        } else if self.negative {
            other.subtract(&self.absolute())
        } else {
            self.subtract(&other.absolute())
        }
    }

    pub fn subtract(&self, other: &BigNumber) -> BigNumber {
        if self.negative != other.negative {
            let sum_digits = self.add_digits(&other.digits);
            BigNumber {
                digits: sum_digits,
                negative: self.negative,
            }
        } else if self >= other {
            let diff_digits = self.subtract_digits(&other.digits);
            BigNumber {
                digits: diff_digits,
                negative: self.negative,
            }
        } else {
            let diff_digits = other.subtract_digits(&self.digits);
            BigNumber {
                digits: diff_digits,
                negative: !self.negative,
            }
        }
    }

    // Multiplication
    pub fn multiply(&self, other: &BigNumber) -> BigNumber {
        let mut result = BigNumber::from_str("0").unwrap();
        for (i, &digit) in other.digits.iter().enumerate() {
            let mut temp = self.multiply_by_digit(digit);
            temp.shift_left(i);
            result = result.add(&temp);
        }
        result.negative = self.negative != other.negative;
        result
    }

    // Division (basic long division algorithm)
    pub fn divide(&self, other: &BigNumber) -> Result<BigNumber, String> {
        if other.is_zero() {
            return Err("Division by zero".to_string());
        }

        let mut result = BigNumber::from_str("0").unwrap();
        let mut remainder = self.clone();

        while remainder >= other.absolute() {
            let mut temp = other.absolute();
            let mut multiple = BigNumber::from_str("1").unwrap();

            while remainder >= temp.multiply_by_digit(10) {
                temp.shift_left(1);
                multiple.shift_left(1);
            }

            remainder = remainder.subtract(&temp);
            result = result.add(&multiple);
        }

        result.negative = self.negative != other.negative;
        Ok(result)
    }

    // Helper: Check if number is zero
    fn is_zero(&self) -> bool {
        self.digits.iter().all(|&d| d == 0)
    }

    // Helper: Return absolute value of BigNumber
    fn absolute(&self) -> BigNumber {
        BigNumber {
            digits: self.digits.clone(),
            negative: false,
        }
    }

    // Helper: Add two sets of digits
    fn add_digits(&self, other: &Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();
        let mut carry = 0;
        let max_len = std::cmp::max(self.digits.len(), other.len());

        for i in 0..max_len {
            let sum = carry + self.digits.get(i).unwrap_or(&0) + other.get(i).unwrap_or(&0);
            carry = sum / 10;
            result.push(sum % 10);
        }

        if carry > 0 {
            result.push(carry);
        }

        result
    }

    // Helper: Subtract two sets of digits (assumes self >= other)
    fn subtract_digits(&self, other: &Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();
        let mut borrow = 0;

        for i in 0..self.digits.len() {
            let mut diff = self.digits[i] as i8 - borrow;
            if i < other.len() {
                diff -= other[i] as i8;
            }

            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.push(diff as u8);
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        result
    }

    // Helper: Multiply by a single digit
    fn multiply_by_digit(&self, digit: u8) -> BigNumber {
        let mut result = Vec::new();
        let mut carry = 0;

        for &d in &self.digits {
            let prod = d as u16 * digit as u16 + carry;
            carry = prod / 10;
            result.push((prod % 10) as u8);
        }

        if carry > 0 {
            result.push(carry as u8);
        }

        BigNumber {
            digits: result,
            negative: false,
        }
    }

    // Helper: Shift digits left (multiply by 10^n)
    fn shift_left(&mut self, n: usize) {
        self.digits.splice(0..0, vec![0; n].into_iter());
    }
}

impl PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.negative != other.negative {
            return if self.negative {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        let len_cmp = self.digits.len().cmp(&other.digits.len());
        if len_cmp != Ordering::Equal {
            return if self.negative {
                len_cmp.reverse()
            } else {
                len_cmp
            };
        }

        for (&a, &b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            let digit_cmp = a.cmp(&b);
            if digit_cmp != Ordering::Equal {
                return if self.negative {
                    digit_cmp.reverse()
                } else {
                    digit_cmp
                };
            }
        }

        Ordering::Equal
    }
}

fn main() {
    let num1 = BigNumber::from_str("222222").unwrap();
    let num2 = BigNumber::from_str("111111").unwrap();

    let sum = num1.add(&num2);
    let difference = num1.subtract(&num2);
    let product = num1.multiply(&num2);
    let quotient = num1.divide(&num2).unwrap();

    println!("Sum: {:?}", sum.to_string());
    println!("Difference: {:?}", difference.to_string());
    println!("Product: {:?}", product.to_string());
    println!("Quotient: {:?}", quotient.to_string());
}
