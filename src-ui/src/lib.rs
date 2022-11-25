use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn SimpleCounter(cx: Scope, name: String) -> Element {
    let (value, set_value) = create_signal(cx, 0);

    let message = create_local_resource(
        cx,
        move || name.clone(),
        move |name| async move {
            let new_msg = invoke("greet", to_value(&GreetArgs { name: &name }).unwrap()).await;

            let new_msg = &new_msg.as_string().unwrap();
            log::info!("{}", &new_msg);

            new_msg.clone()
        },
    );

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear yeah"</button>
            <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>
            <p>{message}</p>
        </div>
    }
}
