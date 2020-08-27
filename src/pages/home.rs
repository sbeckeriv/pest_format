// src/pages/home.rs
use pest_fmt::Settings;
use std::panic;
use yew::prelude::*;
struct State {
    user: String,
    formatted: String,
}
pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    FormatPest(String),
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State {
                user: String::from(""),
                formatted: String::from(""),
            },
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::FormatPest(user_text) => {
                let cfg = Settings::default();
                self.state.formatted = format!("{}", cfg.format(&user_text));
                self.state.user = user_text;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
           <span>
        <form action="#">
           <label for="user">{"User pest input"}</label>
           <textarea rows="33" cols="33" id="user" class="user" value=&self.state.user oninput=self.link.callback(|e: InputData| Msg::FormatPest(e.value)) > </textarea>
           <label for="formatted">{"Formatted pest output"}</label>
           <textarea id="formatted"  name="formatted" class="formatted" rows="33" cols="33" value=&self.state.formatted> </textarea>
           </form>
           </span> }
    }
}
