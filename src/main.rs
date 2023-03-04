mod charindex;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;
use enigma::Enigma;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{html, Component, Context, Event, Html, InputEvent};

pub enum Msg {
    Encrypt(),
    Clear,
    ChangeOriginalString(InputEvent),
    ChangeRotor(u8, Event),
    ChangeRotorPositionOrRingSetting(u8, bool, InputEvent),
    ChangeReflector(Event),
    ChangePlugboard(InputEvent),
}

pub struct App {
    enigma: Enigma,
    most_recent_enigma_config: Enigma,
    original_string: String,
    encrypted_string: String,
}

impl App {
    fn clear(&mut self) {
        self.original_string.clear();
        self.encrypted_string.clear();
    }

    fn encrypt(&mut self) {
        self.encrypted_string = self.enigma.encrypt_string(self.original_string.clone());
    }

    fn get_encrypted_string_html(&self) -> Html {
        html! {
            if self.encrypted_string.len() > 0 {
                <h1>
                {"Encrypted Text:"} { self.encrypted_string.clone()}
               </h1>
            }
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            original_string: String::new(),
            encrypted_string: String::new(),
            enigma: Enigma::new_default_config(),
            most_recent_enigma_config: Enigma::new_default_config(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Encrypt() => {
                self.most_recent_enigma_config = self.enigma.clone();
                self.encrypt();
                true
            }
            Msg::Clear => {
                self.clear();
                true
            }
            Msg::ChangeRotor(rotor_number, event) => {
                let target = event
                    .target()
                    .unwrap()
                    .dyn_ref::<HtmlSelectElement>()
                    .unwrap()
                    .clone();
                let new_rotor_type =
                    target.value().chars().nth(0).unwrap().to_digit(10).unwrap() as u8;

                self.enigma.set_rotor_type(rotor_number, new_rotor_type);
                true
            }
            Msg::ChangeRotorPositionOrRingSetting(rotor_number, is_rotor_position, event) => {
                let target: HtmlInputElement = event
                    .target()
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .clone();
                let new_value = target.value_as_number() as u8;
                if new_value <= 25 {
                    if is_rotor_position {
                        self.enigma.set_rotor_position(rotor_number, new_value);
                    } else {
                        self.enigma.set_ring_setting(rotor_number, new_value);
                    }
                }
                true
            }
            Msg::ChangeReflector(event) => {
                let target = event
                    .target()
                    .unwrap()
                    .dyn_ref::<HtmlSelectElement>()
                    .unwrap()
                    .clone();

                self.enigma.set_reflector(target.value());
                true
            }
            Msg::ChangePlugboard(event) => {
                let target: HtmlInputElement = event
                    .target()
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .clone();
                self.enigma.set_plugboard_mapping(target.value());
                true
            }
            Msg::ChangeOriginalString(event) => {
                let target: HtmlInputElement = event
                    .target()
                    .unwrap()
                    .dyn_ref::<HtmlInputElement>()
                    .unwrap()
                    .clone();
                self.original_string = target.value();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="flex justify-center">
                    <h2>{"Enigma Machine Emulator (Rust/WASM)"}</h2>
                </div>
                <div class="flex flex-column justify-center align-center">
                    <div>
                        <label for="reflector">{"Reflector:"}</label>
                        <select name="reflector" id="reflector" onchange={ctx.link().callback(|e: Event| Msg::ChangeReflector(e))}>
                            <option value="B" selected=true>{"B"}</option>
                            <option value="C">{"C"}</option>
                        </select>
                    </div>
                    <div>
                        <label for="first_rotor">{"Right Rotor:"}</label>
                        <select name="first_rotor" id="first_rotor" onchange={ctx.link().callback(|e: Event| Msg::ChangeRotor(1, e))}>
                            <option value="1" selected=true>{"I"}</option>
                            <option value="2">{"II"}</option>
                            <option value="3">{"III"}</option>
                        </select>
                        <label>{"Position:"}</label>
                        <input type="number" min=0 max=25 placeholder=0 value={self.enigma.get_rotor_position(1).to_string()}  oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeRotorPositionOrRingSetting(1, true, e))}/>
                        <label>{"Ring Setting:"}</label>
                        <input type="number" min=0 max=25 placeholder=0  value={self.enigma.get_ring_setting(1).to_string()} oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeRotorPositionOrRingSetting(1, false, e))}/>
                    </div>
                    <div>
                        <label for="second_rotor">{"Middle Rotor:"}</label>
                        <select name="second_rotor" id="second_rotor" onchange={ctx.link().callback(|e: Event| Msg::ChangeRotor(2, e))}>
                            <option value="1">{"I"}</option>
                            <option value="2" selected=true>{"II"}</option>
                            <option value="3">{"III"}</option>
                        </select>
                        <label>{"Position:"}</label>
                        <input type="number" min=0 max=25 placeholder=0  value={self.enigma.get_rotor_position(2).to_string()}  oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeRotorPositionOrRingSetting(2, true, e))}/>
                        <label>{"Ring Setting:"}</label>
                        <input type="number" min=0 max=25 placeholder=0  value={self.enigma.get_ring_setting(2).to_string()}  oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeRotorPositionOrRingSetting(2, false, e))}/>
                    </div>
                    <div>
                        <label for="third_rotor">{"Left Rotor:"}</label>
                        <select name="third_rotor" id="third_rotor" onchange={ctx.link().callback(|e: Event| Msg::ChangeRotor(3, e))}>
                            <option value="1">{"I"}</option>
                            <option value="2">{"II"}</option>
                            <option value="3" selected=true>{"III"}</option>
                        </select>
                        <label>{"Position:"}</label>
                        <input type="number" min=0 max=25 placeholder=0 value={self.enigma.get_rotor_position(3).to_string()} oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeRotorPositionOrRingSetting(3, true, e))}/>
                        <label>{"Ring Setting:"}</label>
                        <input type="number" min=0 max=25 placeholder=0 value={self.enigma.get_ring_setting(3).to_string()}  oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeRotorPositionOrRingSetting(3, false, e))}/>
                    </div>
                    <div>
                    <div class="flex align-center min-width-30">
                        <label>{"Plugboard Mapping"}</label>
                        <input maxlength="29" oninput={ctx.link().callback(|e: InputEvent| Msg::ChangePlugboard(e))}/>
                    </div>
                    </div>
                </div>
                <div id="buttons" class="flex justify-center">
                    <div class="flex align-center min-width-30">
                        <label>{"Original Text"}</label>
                        <input class="min-width-30" value={self.original_string.clone()} oninput={ctx.link().callback(|e: InputEvent| Msg::ChangeOriginalString(e))}/>
                    </div>
                    <button onclick={ctx.link().callback(|_| Msg::Encrypt())}>
                        { "Encrypt!" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::Clear)}>
                        { "Clear Text!" }
                    </button>
                </div>
                <div class="flex align-center justify-center">
                    {self.get_encrypted_string_html()}
                </div>
            </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
