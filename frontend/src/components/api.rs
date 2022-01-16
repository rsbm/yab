use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use yew::prelude::*;

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

// enum FetchState<T> {
//     NotFetching,
//     Fetching,
//     Success(T),
//     Failed(FetchError),
// }

/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
async fn fetch(url: &str, method: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.body(Some(&r#"{"count": 1}"#.into()));
    opts.method(method);
    // opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;
    request.headers().set("Content-type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

#[function_component(Model)]
pub fn model() -> Html {
    let value = use_state(|| 0);
    // let fetch_state = use_state(|| FetchState::<i32>::NotFetching);

    // use_effect_with_deps(
    //     move |_| {
    //         //

    //         || ()
    //     },
    //     (value, ),
    // );

    let onclick = {
        let value = value.clone();
        Callback::from(move |ev: MouseEvent| {
            ev.prevent_default();
            let value = value.clone();
            spawn_local(async move {
                let api_value = fetch("/hello", "POST").await.unwrap();
                value.set(api_value.parse().unwrap());
            });
        })
    };
    // match &*fetch_state {
    //     FetchState::NotFetching => {
    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{*value}</p>
        </div>
    }
    // }
    // FetchState::Fetching => {
    //     html! {
    //         <p>{"Fetching.."}</p>
    //     }
    // }
    // FetchState::Success(value) => {
    //     html! {
    //         <div>
    //             <button {onclick}>{"+1"}</button>
    //             <p>{value}</p>
    //         </div>
    //     }
    // }
    // FetchState::Failed(err) => {
    //     html! {
    //         <p>{err}</p>
    //     }
    // }
    // }
}
