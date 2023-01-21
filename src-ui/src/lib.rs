use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};
use tauri_sys::{event, tauri};

#[derive(Serialize)]
struct GreetCmdArgs {
    name: String,
}

#[derive(Serialize)]
struct EmitEventCmdArgs {
    num: u16,
}

#[derive(Debug, Deserialize)]
struct GreetEventRes {
    greeting: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
struct GenericEventRes {
    num: u16,
    message: String,
}

async fn greet(name: String) -> String {
    tauri::invoke("greet", &GreetCmdArgs { name })
        .await
        .unwrap()
}

async fn listen_on_greet_event() -> String {
    let event = event::once::<GreetEventRes>("greet-event").await.unwrap();
    log::debug!("Received greet-event {:#?}", event);
    event.payload.greeting
}

async fn emit_generic_event(num: u16) {
    tauri::invoke::<_, ()>("emit_event", &EmitEventCmdArgs { num })
        .await
        .unwrap();
}

async fn listen_on_generic_event(event_writer: WriteSignal<Vec<GenericEventRes>>) {
    let mut events = event::listen::<GenericEventRes>("generic-event")
        .await
        .unwrap();

    while let Some(event) = events.next().await {
        log::debug!("Received generic-event {:#?}", event);
        event_writer.update(|all_events| all_events.push(event.payload));
    }
}

#[component]
pub fn SimpleCounter(cx: Scope, name: String) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);

    // Greet event, will clean-up once event is received.
    let (greet_event_msg, set_greet_event_msg) =
        create_signal(cx, "No `greet-event` from Tauri.".to_string());

    let greet_event_resource = create_local_resource(cx, move || (), |_| listen_on_greet_event());
    let greet_event_msg_memo = create_memo(cx, move |_| {
        set_greet_event_msg(
            greet_event_resource
                .read()
                .unwrap_or("Waiting for `greet-event` from Tauri.".to_string()),
        );
    });
    create_effect(cx, move |_| greet_event_msg_memo);

    // Generic event, listening constantly.
    let (event_counter, set_event_counter) = create_signal(cx, 1u16);
    let (event_vec, set_event_vec) = create_signal::<Vec<GenericEventRes>>(cx, vec![]);
    let emit_event_action = create_action(cx, |num: &u16| emit_generic_event(*num));
    create_local_resource(cx, move || set_event_vec, listen_on_generic_event);

    // Greet command response.
    // `greet` commands emits `greet-event`. It has to be called after `listen_on_greet_event`.
    // (In order to make sure the once event has been hooked up.)
    let msg = create_local_resource(cx, move || name.to_owned(), greet);

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>

            <p>{msg}</p>
            <p>{greet_event_msg}</p>

            <button on:click=move |_| {
                emit_event_action.dispatch(event_counter());
                set_event_counter(event_counter() + 1);
            }>"Emit generic event"</button>

            <ul>
                <For each=event_vec key=|e| e.num view=move |e: GenericEventRes| {
                    view! {
                        cx,
                        <li>{e.message.clone()}</li>
                    }
                } />
            </ul>
        </div>
    }
}
