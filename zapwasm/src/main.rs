
use std::rc::Rc;

use yew::prelude::*;

mod components;
mod tools;

use crate::tools::logger::log;
use crate::components::inputfield::Inputfield;

struct MainComponent {
    persona: String,
    excuse_for: String,
    excuse: String,
}

enum MainMsg {
    UpdatePersona(AttrValue),
    UpdateExcuseFor(AttrValue),
    GenerateExcuse,
}

#[derive(Default, Clone, Properties, PartialEq)]
struct MainProperties {
    pub persona: AttrValue,

    pub excuse_for: AttrValue,

    pub excuse: AttrValue,
}

impl Component for MainComponent {
    type Message = MainMsg;
    type Properties = MainProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            persona: ctx.props().persona.to_string(), 
            excuse_for: ctx.props().excuse_for.to_string(), 
            excuse: ctx.props().excuse.to_string() 
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MainMsg::UpdatePersona(persona) => {
                log(&format!("Persona: {}", persona.to_string()));
                self.persona = persona.to_string();
                true
            }
            MainMsg::UpdateExcuseFor(excuse_for) => {
                log(&format!("Excuse for: {}", excuse_for.to_string()));
                self.excuse_for = excuse_for.to_string();
                true
            }
            MainMsg::GenerateExcuse => {
                log(&format!("{} is sorry for the {}", self.persona, self.excuse_for));
                self.excuse = self.generate_excuse();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <div class="main">
                <h1>{"LED ZAP!"}</h1>
                <Inputfield label="Persona" 
                            text={ ctx.props().persona.clone() }
                            onchange={ ctx.link().callback(move |persona: String| MainMsg::UpdatePersona(AttrValue::from(persona))) }
                />
                <Inputfield label="Excuse for"
                            text={ ctx.props().excuse_for.clone() }
                            onchange={ ctx.link().callback(move |excuse_for: String| MainMsg::UpdateExcuseFor(AttrValue::from(excuse_for))) }
                />
                <div class="excuse_field">
                <div>{ format!("{}", self.excuse) }</div>
                </div>
                <button class="primary_button" onclick={ ctx.link().callback(|_| MainMsg::GenerateExcuse) }>
                    {"Generate lame excuse"}
                </button>
            </div>
        }
    }
}

impl MainComponent {
    fn generate_excuse(&self) -> String {
        let excuse = format!("{} is sorry for the {}", self.persona, self.excuse_for);
        excuse
    }
}

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
            let excuse = format!("{} is sorry for the {}", rcx.persona, rcx.excuse_for);
            rcx.dispatch(MainStateAction::GenerateExcuse(excuse));
            ()
        })
    };

    html! {
        <div class="main">
            <h1>{"LED ZAP!"}</h1>
            <Inputfield label="Persona" 
                        text={ rc.persona.clone() }
                        onchange={ on_persona_change }
            />
            <Inputfield label="Excuse for"
                        text={ rc.excuse_for.clone() }
                        onchange={ on_excuse_for_change }
            />
            <div class="excuse_field">
            <div>{ format!("{}", rc.excuse.clone().to_string()) }</div>
            </div>
            <button class="primary_button" onclick={ on_generate_excuse }>
                {"Generate lame excuse"}
            </button>
        </div>
    }
}

/** Main */
fn main() {
    log(&"Hello, world!".to_string());
    yew::Renderer::<MainFunctionComponent>::new().render();
}

