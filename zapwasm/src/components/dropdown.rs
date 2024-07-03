use yew::prelude::*;

pub struct DropdownComponent {
}

pub enum DropdownMsg {
}

#[derive(Clone, Properties, PartialEq)]
pub struct DropdownProps {
    pub label: Option<String>,
}

impl Component for DropdownComponent {
    type Message = DropdownMsg;
    type Properties = DropdownProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let label = ctx.props().label.clone().unwrap_or("Choose a persona:".to_string());

        html! {
            <div class="drowpdown">
                <label for="dropdown">{ label }</label>
                <select name="dropdown" id="dropdown">
                    <option value="persona1">{"Persona 1"}</option>
                    <option value="persona2">{"Persona 2"}</option>
                    <option value="persona3">{"Persona 3"}</option>
                </select>
            </div>
        }
    }
}
