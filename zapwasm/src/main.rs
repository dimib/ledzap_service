use std::rc::Rc;

use yew::prelude::*;

use gloo_net::http::{Request, Headers};
use wasm_bindgen_futures::spawn_local;

mod components;
mod tools;
mod client;

use crate::tools::logger::log;
use crate::components::inputfield::Inputfield;
use crate::client::model::ExcuseResponse;

// -- Main Function Component --

struct MainState {
    pub persona: String,
    pub excuse_for: String,
    pub excuse: String,
}

impl Default for MainState {
    fn default() -> Self {
        Self {
            persona: String::from("Bla"),
            excuse_for: String::from("Fasel"),
            excuse: String::from(""),
        }
    }
}

enum MainStateAction {
    UpdatePersona(String),
    UpdateExcuseFor(String),
    GenerateExcuse(String),
}

impl Reducible for MainState {
    type Action = MainStateAction;
    
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MainStateAction::UpdatePersona(persona) => {
                log(&format!("reduce Persona: {}", persona));
                let state = self;
                Rc::new(MainState { persona: persona, excuse_for: state.excuse_for.clone(), excuse: state.excuse.clone() })
            }
            MainStateAction::UpdateExcuseFor(excuse_for) => {
                log(&format!("reduce Excuse for: {}", excuse_for));
                let state = self;
                Rc::new(MainState { persona: state.persona.clone(), excuse_for: excuse_for.clone(), excuse: state.excuse.clone() })
            }
            MainStateAction::GenerateExcuse(excuse) => {
                log(&format!("{} is sorry for the {}", self.persona, self.excuse_for));
                let state = self;
                Rc::new(MainState { persona: state.persona.clone(), excuse_for: state.excuse_for.clone(), excuse: excuse.clone() })
            }
        }
    }
}


#[function_component]
fn MainFunctionComponent() -> Html {

    let main_state = use_reducer(MainState::default);
    let rc = Rc::new(main_state);

    let on_persona_change = {
        let rcx = Rc::clone(&rc);
        Callback::from(move |persona: String| {
            rcx.dispatch(MainStateAction::UpdatePersona(persona));
            ()
        })
    };

    let on_excuse_for_change = {
        let rcx = Rc::clone(&rc);
        Callback::from(move |excuse_for: String| {
            rcx.dispatch(MainStateAction::UpdateExcuseFor(excuse_for));
            ()
        })
    };

    let on_generate_excuse = {
        let rcx = Rc::clone(&rc);
        Callback::from(move |_e: MouseEvent| {
            let query = [("persona", rcx.persona.clone()), ("topic", rcx.excuse_for.clone())];
            let headers = Headers::new();
            headers.append("x-api-key", "abxx-1234-5678-9abc");

            let rcx2 = Rc::clone(&rcx);

            spawn_local(async move {
                let excuse: ExcuseResponse = Request::get("http://127.0.0.1:3003/excuse/")
                    .query(query)
                    .header("x-api-key", "abxx-1234-5678-9abc")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                log(&format!("Excuse: {}", excuse.excuse.clone()));
                rcx2.dispatch(MainStateAction::GenerateExcuse(excuse.excuse.clone()));
            });
            ()
        })
    };

    html! {
        <div class="main">
            <h1>{"LED ZAP!"}</h1>
            <Inputfield label="Persona" 
                        hint="Schlechter Schüler"
                        text={ rc.persona.clone() }
                        onchange={ on_persona_change }
            />
            <Inputfield label="Excuse for"
                        hint="Keine Hausaufgaben"
                        text={ rc.excuse_for.clone() }
                        onchange={ on_excuse_for_change }
            />
            <div class="excuse_field">
                <div>{ format!("{}", rc.excuse.clone().to_string()) }</div>
            </div>
            <div class="button_area">
                <button class="primary_button" onclick={ on_generate_excuse }>
                    {"Generate lame excuse"}
                </button>
            </div>
        </div>
    }
}

/** Main */
fn main() {
    log(&"Hello, world!".to_string());
    yew::Renderer::<MainFunctionComponent>::new().render();
}

