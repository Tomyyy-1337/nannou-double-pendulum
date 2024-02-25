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
    let height = 700;
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
        model.update_physics(update, 100);
        model.upate_trace();
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);

    draw_model(&draw, model);
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}


fn draw_model(draw: &Draw, model: &Model) {
    let origin = model.origin;
    let x1 = origin.x + model.r1 * -model.a1.sin();
    let y1 = origin.y + model.r1 * -model.a1.cos();
    let x2 = x1 + model.r2 * -model.a2.sin();
    let y2 = y1 + model.r2 * -model.a2.cos();
    let color = TEAL;

    draw.ellipse()
        .x_y(x1, y1)
        .radius(model.m1.sqrt())
        .color(color)
        .z(3.0);
    draw.ellipse()
        .x_y(x2, y2)
        .radius(model.m2.sqrt())
        .color(color)
        .z(3.0);
    draw.line()
        .start(origin)
        .end(pt2(x1, y1))
        .color(color)
        .weight(3.0)
        .z(2.0);
    draw.line()
        .start(pt2(x1, y1))
        .end(pt2(x2, y2))
        .color(color)
        .weight(3.0)
        .z(2.0);

    draw.polyline()
        .weight(1.0)
        .z(1.0)
        .points_colored(
            model.trace
                .iter()
                .enumerate()
                .map(|(i,p)| (*p, lin_srgba(1.0, 1.0, 1.0, 1.0 - (model.trace.len() - i) as f32 / model.trace.len() as f32)))
        );

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}