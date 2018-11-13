extern crate azul;

use azul::{prelude::*, widgets::{label::Label, button::Button}};

struct MyDataModel {
    counter: usize,
}

fn update_counter(app_state: &mut AppState<MyDataModel>, _event: WindowEvent<MyDataModel>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    UpdateScreen::Redraw
}

impl Layout for MyDataModel {
    fn layout(&self, _: WindowInfo<Self>) -> Dom<Self> {
        let lbl = Label::new(format!("{}", self.counter))
            .dom();
        let btn = Button::with_label("Update counter")
            .dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        Dom::new(NodeType::Div)
            .with_child(lbl)
            .with_child(btn)
    }
}

fn main() {
    let app = App::new(MyDataModel { counter: 0 }, AppConfig::default());
    let window = Window::new(WindowCreateOptions::default(), Css::native()).unwrap();
    app.run(window).unwrap();
}