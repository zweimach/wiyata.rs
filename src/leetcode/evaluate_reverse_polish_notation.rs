use std::str::FromStr;

enum Operator {
    Add,
    Substitute,
    Multiply,
    Divide,
}

impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Substitute),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(""),
        }
    }
}

impl Operator {
    pub fn apply(self, rhs: i32, lhs: i32) -> i32 {
        match self {
            Self::Add => rhs + lhs,
            Self::Substitute => rhs - lhs,
            Self::Multiply => rhs * lhs,
            Self::Divide => (rhs as f64 / lhs as f64).trunc() as i32,
        }
    }
}

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    *tokens
        .iter()
        .fold(vec![], |mut stack, curr| {
            if let Ok(op) = curr.parse::<Operator>() {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                stack.push(op.apply(num1, num2));
            } else {
                stack.push(curr.parse().unwrap());
            }
            stack
        })
        .first()
        .unwrap()
}
