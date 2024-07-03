
use yew::prelude::*;
use web_sys::HtmlInputElement;
use crate::tools::logger::log;

#[derive(Clone, Properties, PartialEq)]
pub struct InputfieldProps {
    pub label: AttrValue,
    pub text: AttrValue,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn Inputfield(props: &InputfieldProps) -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();
    let my_props = props.to_owned();

    let on_change = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                let new_value = input.value();
                input_value_handle.set(new_value.clone());
                my_props.onchange.emit(new_value.clone());
                log(&format!("Input value: {}", new_value.clone()));
            }
        })
    };

    html! {
        <div class="inputfield">
            <label for="input">{ props.label.clone() }</label>
            <input type="text"
                id="input"
                onchange={ on_change }
                value={ input_value.clone() }
            />
        </div>
    }
}