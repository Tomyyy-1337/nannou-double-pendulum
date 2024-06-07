use nannou::prelude::*;

mod model;
use model::Model;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let width = 1000;
    let height = 1000;
    app.new_window()
        .size(width, height)
        .view(view)
        .build()
        .unwrap();

    Model::new(width, height)
}

fn update(app: &App, model: &mut Model, update: Update) {
    let window_width = app.window_rect().w() as u32;
    let window_height = app.window_rect().h() as u32;
    
    if window_width != model.window_width || window_height != model.window_height {
        *model = Model::new(window_width, window_height);
    }
    app.keys.down.iter().for_each(|key| {
        match key {
            Key::Space => model.clear_trace(),
            Key::Return => model.reset(),
            Key::F11 => app.main_window().set_fullscreen(!app.main_window().is_fullscreen()),
            Key::Back => model.running = !model.running,
            _ => (),
        }
    });

    if model.running {
        // if model.chaos_factor() > 0.3 {
        //     model.reset_timer += 1;
        //     if model.reset_timer > 2000 {
        //         model.reset();
        //     }
        // }
        model.update_physics(update, 100);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.rect()
        .w_h(app.window_rect().w(), app.window_rect().h())
        .color(Srgba::new(0.0, 0.0, 0.0, 0.015));

    model.draw(&draw);
    
    draw.to_frame(app, &frame).unwrap();
}