use azul::{
    prelude::*,
    widgets::{
        label::Label,
        button::Button,
        text_input::TextInput,
        text_input::TextInputState,
    },
};

struct MyDataModel {
    counter: usize,
    txt_state: TextInputState,
}

fn update_counter(app_state: &mut AppState<MyDataModel>, _event: WindowEvent<MyDataModel>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    UpdateScreen::Redraw
}

impl Layout for MyDataModel {
    fn layout(&self, info: WindowInfo<Self>) -> Dom<Self> {
        let lbl = Label::new(format!("{}", self.counter))
            .dom();
        let btn = Button::with_label("Update counter")
            .dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        let txt: Dom<MyDataModel> = TextInput::new()
            .bind(info.window, &self.txt_state, &self)
            .dom(&self.txt_state);

        Dom::new(NodeType::Div)
            .with_child(lbl)
            .with_child(btn)
            .with_child(txt)
    }
}

fn main() {
    let dm = MyDataModel {
        counter: 0,
        txt_state: TextInputState::new("Hello?")
    };
    let app = App::new(dm, AppConfig::default());
    let window = Window::new(WindowCreateOptions::default(), Css::native()).unwrap();
    app.run(window).unwrap();
}