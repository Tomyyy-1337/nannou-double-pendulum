use nannou::prelude::*;

mod model;
use model::Model;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let width = 1200;
    let height = 800;
    let window_id = app
        .new_window()
        .size(width, height)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    Model::new(&window, width, height)
}

fn update(app: &App, model: &mut Model, update: Update) {
    let window_width = app.window_rect().w() as u32;
    let window_height = app.window_rect().h() as u32;
    
    if window_width != model.window_width || window_height != model.window_height {
        *model = Model::new(&app.main_window(),window_width, window_height);
    }
    app.keys.down.iter().for_each(|key| {
        match key {
            Key::Space => model.trace.clear(),
            Key::Return => model.reset(),
            Key::F11 => app.main_window().set_fullscreen(!app.main_window().is_fullscreen()),
            Key::Back => model.running = !model.running,
            _ => (),
        }
    });

    model.update_gui(update);

    if model.running {
        model.update_physics(update, 1000);
        model.upate_trace();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);

    model.draw(&draw);
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}