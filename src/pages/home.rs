// src/pages/home.rs
use pest_fmt::Settings;
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
impl Home {
    fn header(&self) -> Html {
        html! {
            <div id="nescss">
              <header class="{ sticky: scrollPos > 50 }">
                <div class="container">
                  <div class="nav-brand">
                    <h1><img src="https://raw.githubusercontent.com/sbeckeriv/pest_format/master/docs/logo.gif" height="50"/>{" Pest Formatter"}</h1>
                  </div>

                  <div class="social-buttons">
                  </div>
                </div>
              </header>
            </div>
        }
    }
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
            <>
            <div id="nescss">
            {self.header()}
           <div class="half">
           <label for="user">{"User pest input"}</label>
           <textarea rows="20" cols="33" id="user" class="user nes-textarea" value=&self.state.user oninput=self.link.callback(|e: InputData| Msg::FormatPest(e.value)) > </textarea>
           </div>
           <div class="half">
           <label for="formatted">{"Formatted pest output"}</label>
           <textarea id="formatted"  name="formatted" class="formatted nes-textarea" rows="20" cols="33" value=&self.state.formatted> </textarea>
           </div>
           </div>
           </>
        }
    }
}
