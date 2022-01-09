use yew::prelude::*;

#[function_component(Model)]
pub fn model() -> Html {
    let value = use_state(|| 0);

    let onclick = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value + 1))
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{*value}</p>
        </div>
    }
}
