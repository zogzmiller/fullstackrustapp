use crate::{
    api::api_create_user,
    store::{set_user, set_loading, set_show_alert, Store},
};
use common::User;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
pub fn LoginForm() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let loading = &store.loading;
    let text = use_state(String::new);
    let min = use_state(|| 10);
    let message = use_state(|| Option::<String>::None);

    let text_input_ref = use_node_ref();

    let handle_input = {
        let text = text.clone();
        let message = message.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            message.set(None);
            text.set(value);
        })
    };

    let on_submit = {
        let cloned_dispatch = dispatch.clone();
        let cloned_text_input_ref = text_input_ref.clone();
        let cloned_text = text.clone();
        let message = message.clone();

        Callback::from(move |event: SubmitEvent| {
            let text_input_ref = cloned_text_input_ref.clone();
            let text = cloned_text.clone();
            let dispatch = cloned_dispatch.clone();

            event.prevent_default();
            set_loading(true, dispatch.clone());

            if text.trim().len() < *min {
                message.set(Some("Text must be at least 10 characters".to_string()));
                set_loading(false, dispatch.clone());
                return;
            }

            let user_data = serde_json::json!({
                "text": text.to_string()
            });

            spawn_local(async move {
                set_loading(true, dispatch.clone());
                let text_input = text_input_ref.cast::<HtmlInputElement>().unwrap();
                text_input.set_value("");
                text.set(String::new());

                let response: Result<User, String> = api_create_user(user_data.to_string().as_str()).await;

                match response {
                    Ok(user) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert("Feeback added successfully".to_string(), dispatch.clone());
                        set_user(user, dispatch);
                    }
                    Err(e) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            });
        })
    };

    html! {
        <div class="bg-white text-gray-700 rounded-lg p-8 my-5 relative">
            <header class="max-w-md mx-auto">
                <h2 class="text-center text-2xl font-bold">{"How would you rate your service with us?"}</h2>
            </header>
            <form onsubmit={on_submit}>
                <div class="flex border rounded-lg my-4 px-2 py-3">
                    <input
                        type="text"
                        ref={text_input_ref}
                        oninput={handle_input}
                        class="flex-grow border-none text-lg focus:outline-none"
                        placeholder="Tell us something that keeps you coming back"
                    />
                <button
                    type="submit"
                    class={format!(
                        "border-0 rounded-md w-28 h-10 cursor-pointer hover:bg-indigo-500 {}",
                        if *loading { "bg-[#ccc] text-gray-800"} else {"bg-indigo-600 text-white"}
                    )}
                >
                    {"Send"}
                </button>
                </div>
                {if let Some(msg) = message.as_ref() {
                    html! { <div class="pt-3 text-center text-purple-600">{msg.clone()}</div> }
                } else {
                    html! {}
                }}
            </form>
        </div>
    }
}