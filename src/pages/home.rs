// src/pages/home.rs
use pest_fmt::{PestError, Settings};
use yew::prelude::*;
struct State {
    pub user: String,
    pub formatted: String,
    // from settings
    pub indent: usize,
    pub choice_hanging: bool,
    pub choice_first: bool,
    pub set_alignment: bool,
    // spaces between `=`
    pub set_space: usize,
    // spaces between `|`
    pub choice_space: usize,
    // spaces between `{ }`
    pub braces_space: usize,
    // spaces between `~`
    pub sequence_space: usize,
    // spaces between `( )`
    pub parentheses_space: usize,
}

impl Default for State {
    fn default() -> Self {
        State {
            user: String::from("alpha={ 'a'..'z' | 'A'..'Z' }\ndigit={ '0'..'9' }\n\nident = { (alpha | digit)+ }\nident_list = _{ !digit ~ ident ~ (\" \" ~ ident)+ }"),
            formatted: String::from("alpha =  {'a'..'z'|'A'..'Z'}\ndigit =  {'0'..'9'}\n\nident      =  {(alpha|digit)+}\nident_list = _{!digit ~ ident ~ (\" \" ~ ident)+}"),
            // tab = 4 space
            indent: 4,
            set_alignment: true,
            choice_first: true,
            choice_hanging: false,
            set_space: 1,
            choice_space: 0,
            braces_space: 0,
            sequence_space: 1,
            parentheses_space: 0,
        }
    }
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
}

pub enum Msg {
    FormatPest(String),
    SetIndent(usize),
    ChoiceHanging,
    ChoiceFirst,
    SetAlignment,
    // spaces between `=`
    SetSpace(usize),
    // spaces between `|`
    ChoiceSpace(usize),
    // spaces between `{ }`
    BracesSpace(usize),
    // spaces between `~`
    SequenceSpace(usize),
    // spaces between `( )`
    ParenthesesSpace(usize),
}
impl Home {
    fn fmt(&mut self) {
        let cfg = self.formatter();
        let current_user = &self.state.user.clone();

        match cfg.format(&current_user) {
            Ok(formatted) => {
                self.state.formatted = formatted;
            }
            Err(error) => match error {
                PestError::FormatFail(e) => {
                    self.state.formatted = format!("Error formatting {}", e);
                }

                PestError::ParseFail(e) => {
                    self.state.formatted = e;
                }
                _ => {}
            },
            _ => {}
        };
    }
    fn formatter(&self) -> Settings {
        Settings {
            indent: self.state.indent,
            set_alignment: self.state.set_alignment,
            blank_lines: None,
            choice_first: self.state.choice_first,
            choice_hanging: self.state.choice_hanging,
            set_space: self.state.set_space,
            choice_space: self.state.choice_space,
            braces_space: self.state.braces_space,
            sequence_space: self.state.sequence_space,
            parentheses_space: self.state.parentheses_space,
        }
    }
    fn turtles(&self) -> Html {
        if self.state.choice_hanging {
            html! {
                <>
                    <i style="float:right;position: relative; top: -67px;" class="nes-squirtle animate is-small"></i>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        }
    }
    fn header(&self) -> Html {
        html! {
            <header class="{ sticky: scrollPos > 50 }">
                <div class="container">
                <div class="nav-brand">
                <h1><img src="https://raw.githubusercontent.com/sbeckeriv/pest_format/master/docs/logo.gif" height="50"/>{" Pest Formatter"}</h1>
                </div>

                <div class="social-buttons">
                </div>
                </div>
                </header>
        }
    }

    fn footer(&self) -> Html {
        html! {
            <div id="footer" style="clear:both; width: 62%; margin:20px">
                <section class="nes-container with-title">
                <h3 class="title">{"Thanks"}</h3>
                <section class="message-list">
                <section class="message -left">
                <i class="nes-ash animate is-small"></i>
                <div class="nes-balloon from-left">
                <p>{"Thanks to "} <a href="https://pest.rs/" target="_blank">{"Pest"}</a> {" and "} <a href="https://github.com/pest-parser/pest-fmt" target="_blank">{ "pest-fmt" }</a></p>
                </div>
                </section>
                <section class="message -right">
                <div class="nes-balloon from-right">
                <p><a href="https://github.com/sbeckeriv/pest_format" target="_blank">{ "Github repo" }</a></p>
                </div>
                <i class="nes-octocat is-small"></i>
                </section>

                <section class="message -left">
                <i class="nes-ash animate is-small"></i>
                <div class="nes-balloon from-left">
                <p><a href="https://nostalgic-css.github.io/NES.css/" target="_blank">{"NES.css"}</a> {" and "} <a href="https://github.com/yewstack/yew" target="_blank">{ "yew" }</a></p>
                </div>
                </section>
                </section>
                </section>
                </div>
        }
    }
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State::default(),
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::FormatPest(user_text) => {
                self.state.user = user_text;
                self.fmt();
                true
            }
            Msg::SetIndent(number) => {
                self.state.indent = number;
                self.fmt();
                true
            }
            Msg::SetSpace(number) => {
                self.state.set_space = number;
                self.fmt();
                true
            }
            Msg::ChoiceSpace(number) => {
                self.state.choice_space = number;
                self.fmt();
                true
            }
            Msg::BracesSpace(number) => {
                self.state.braces_space = number;
                self.fmt();
                true
            }
            Msg::SequenceSpace(number) => {
                self.state.sequence_space = number;
                self.fmt();
                true
            }
            Msg::ParenthesesSpace(number) => {
                self.state.parentheses_space = number;
                self.fmt();
                true
            }

            Msg::ChoiceHanging => {
                self.state.choice_hanging = !self.state.choice_hanging;
                self.fmt();
                true
            }

            Msg::ChoiceFirst => {
                self.state.choice_first = !self.state.choice_first;
                self.fmt();
                true
            }
            _ => true,
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
                        <div class="settings nes-container with-title" style="clear:both; margin:20px;width: 62%; height:160px">
                        <h3 class="title">{"Settings"}</h3>
                        <div class="nes-field indent">
                        <label for="indent">{ "Indent" }</label>
                        <input type="number" id="indent" class="nes-input" value=&self.state.indent oninput=self.link.callback(|e: InputData| Msg::SetIndent(e.value.parse::<usize>().unwrap_or(0))) />
                        </div>
        /*
                        <div class="nes-field setspace">
                        <label for="setspace">{ "= Spacing" }</label>
                        <input type="number" id="setspace" class="nes-input" value=&self.state.set_space oninput=self.link.callback(|e: InputData| Msg::SetSpace(e.value.parse::<usize>().unwrap_or(0))) />
                        </div>

                        <div class="nes-field choicespace">
                        <label for="choicespace">{ "| Spacing" }</label>
                        <input type="number" id="choicespace" class="nes-input" value=&self.state.choice_space oninput=self.link.callback(|e: InputData| Msg::ChoiceSpace(e.value.parse::<usize>().unwrap_or(0))) />
                        </div>

                        <div class="nes-field bracesspace">
                        <label for="bracesspace">{ "{} Spacing" }</label>
                        <input type="number" id="bracesspace" class="nes-input" value=&self.state.braces_space oninput=self.link.callback(|e: InputData| Msg::BracesSpace(e.value.parse::<usize>().unwrap_or(0))) />
                        </div>
        */
                        <div class="nes-field sequencespace">
                        <label for="sequencespace">{ "~" }</label>
                        <input type="number" id="sequencespace" class="nes-input" value=&self.state.sequence_space oninput=self.link.callback(|e: InputData| Msg::SequenceSpace(e.value.parse::<usize>().unwrap_or(0))) />
                        </div>
        /*
                        <div class="nes-field parenthesesspace">
                        <label for="parenthesesspace">{ "() Spacing" }</label>
                        <input type="number" id="parenthesesspace" class="nes-input" value=&self.state.parentheses_space oninput=self.link.callback(|e: InputData| Msg::ParenthesesSpace(e.value.parse::<usize>().unwrap_or(0))) />
                        </div>
        */
                        <div class="nes-field choicefirst">
                        <label>
                          <input type="checkbox" class="nes-checkbox" checked=self.state.choice_first onclick=self.link.callback(|_| Msg::ChoiceFirst) />
                          <span>{"Choice before the pipe(|)"}</span>
                        </label>
                        </div>

                        <div class="nes-field hangingfirst">
                        <label>
                          <input type="checkbox" class="nes-checkbox" checked=self.state.choice_hanging onclick=self.link.callback(|_| Msg::ChoiceHanging) />
                          <span>{"I like turtles"}</span>
                        </label>
                        </div>
                        { self.turtles() }

                        </div>
                        <br/>
                        {self.footer()}
                    </div>
                        </>
                }
    }
}
