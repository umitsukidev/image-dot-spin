use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .primary()
        .size(1200, 900)
        .resizable(false)
        .view(view)
        .build();

    Model {}
}

fn update(_app: &App, _model: &mut Model) {}

fn view(_app: &App, _model: &Model) {}
