pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32, String> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32, String> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                // `context` メソッドを `ok_or_else` で代用
                let y = stack
                    .pop()
                    .ok_or_else(|| format!("invalid syntax at {}", pos))?;
                let x = stack
                    .pop()
                    .ok_or_else(|| format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    // `bail!` マクロを `return Err(...)` で書き直す`
                    _ => return Err(format!("invalid token at {}", pos)),
                };
                stack.push(res);
            }

            // `-v`オプションが指定されている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        // `ensure!` マクロを `if` 文で書き直す
        if stack.len() != 1 {
            return Err("invalid syntax".to_string());
        }

        Ok(stack[0])
    }
}
