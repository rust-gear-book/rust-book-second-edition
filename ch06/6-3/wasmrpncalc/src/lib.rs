use wasm_bindgen::prelude::*;
mod rpn;

use rpn::RpnCalculator;

#[wasm_bindgen]
pub struct WasmRpnCalculator {
    inner: RpnCalculator,
}

#[wasm_bindgen]
impl WasmRpnCalculator {
    #[wasm_bindgen(constructor)]
    pub fn new(verbose: bool) -> Self {
        Self {
            inner: RpnCalculator::new(verbose),
        }
    }

    pub fn eval(&self, formula: &str) -> Result<i32, JsValue> {
        self.inner.eval(formula).map_err(|e| JsValue::from_str(&e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[cfg(not(feature = "benchmark"))]
    #[wasm_bindgen_test]
    fn test() {
        let calc = WasmRpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[cfg(feature = "benchmark")]
    #[wasm_bindgen_test]
    fn wasm_bench() {
        let calc = WasmRpnCalculator::new(false);
        for _ in 0..100000000 {
            let _ = calc.eval("2 3 *");
        }
    }

    #[cfg(feature = "benchmark")]
    #[test]
    fn native_bench() {
        let calc = WasmRpnCalculator::new(false);
        for _ in 0..100000000 {
            let _ = calc.eval("2 3 *");
        }
    }
}
