mod rpn;

use leptos::prelude::*;
use rpn::RpnCalculator;

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let (result, set_result) = signal(String::new());
    let calc = RpnCalculator::new(false);

    let calculate = move |_| {
        let formula = value.get();

        let res = calc
            .eval(&formula)
            .map(|result| result.to_string())
            .unwrap_or_else(|e| e);

        set_result.set(res);
    };

    view! {
        <div>
            <h1>RPN Calculator</h1>
            <input type="text"
                value={value}
                on:input:target={move |e| set_value.set(e.target().value())}
                placeholder="Enter a formula"
            />
            <button on:click=calculate>Calculate</button>
            <p>{result}</p>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! { <App /> })
}
