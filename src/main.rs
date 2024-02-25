use std::collections::VecDeque;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

pub struct Model {
    window_width: u32,
    window_height: u32,
    r1: f32,
    r2: f32,
    m1: f32,
    m2: f32,
    a1: f32,
    a2: f32,
    a1_v: f32,
    a2_v: f32,
    g: f32,
    trace: VecDeque<Point2>,
}

fn model(app: &App) -> Model {
    let width = app.window_rect().w() as u32;
    let height = app.window_rect().h() as u32;
    Model::new(width, height)
}

impl Model {
    pub fn new(width: u32, height: u32) -> Self {
        Model {
            window_width: width,
            window_height: height,
            r1: 250.0,
            r2: 240.0,
            m1: 80.0,
            m2: 40.0,
            a1: PI / 2.0 + 0.1,
            a2: PI / 2.0 + 0.2,
            a1_v: 0.0,
            a2_v: 0.0,
            g: 10.0,
            trace: VecDeque::with_capacity(5010),
        }
    }

    fn update(&mut self, update: Update) {
        let delta_time = update.since_last.secs() as f32 * 10.0;

        let num1 = -self.g * (2.0 * self.m1 + self.m2) * self.a1.sin();
        let num2 = -self.m2 * self.g * (self.a1 - 2.0 * self.a2).sin();
        let num3 = -2.0 * (self.a1 - self.a2).sin() * self.m2;  
        let num4 = self.a2_v.powi(2) * self.r2 + self.a1_v.powi(2) * self.r1 * (self.a1 - self.a2).cos();
        let den = self.r1 * (2.0 * self.m1 + self.m2 - self.m2 * (2.0 * self.a1 - 2.0 * self.a2).cos());    
        let a1_a = (num1 + num2 + num3 * num4) / den;

        let num1 = 2.0 * (self.a1 - self.a2).sin();
        let num2 = self.a1_v.powi(2) * self.r1 * (self.m1 + self.m2);
        let num3 = self.g * (self.m1 + self.m2) * self.a1.cos();
        let num4 = self.a2_v.powi(2) * self.r2 * self.m2 * (self.a1 - self.a2).cos();
        let den = self.r2 * (2.0 * self.m1 + self.m2 - self.m2 * (2.0 * self.a1 - 2.0 * self.a2).cos());
        let a2_a = (num1 * (num2 + num3 + num4)) / den;

        self.a1_v += a1_a * delta_time;
        self.a2_v += a2_a * delta_time;

        self.a1 += self.a1_v * delta_time;
        self.a2 += self.a2_v * delta_time;
        
        let origin = pt2(0.0, (self.r1 + self.r2) / 4.0);
        let x1 = origin.x + self.r1 * -self.a1.sin() as f32;
        let y1 = origin.y + self.r1 * -self.a1.cos() as f32;
        let x2 = x1 + self.r2 * -self.a2.sin() as f32;
        let y2 = y1 + self.r2 * -self.a2.cos() as f32;

        self.trace.push_back(pt2(x2, y2));
        if self.trace.len() > 5000 {
            self.trace.pop_front();
        }
    }
}


fn update(app: &App, model: &mut Model, update: Update) {
    let window_width = app.window_rect().w() as u32;
    let window_height = app.window_rect().h() as u32;
    
    if window_width != model.window_width || window_height != model.window_height {
        *model = Model::new(window_width, window_height);
    }
    app.keys.down.iter().for_each(|key| {
        match key {
            Key::Space => model.trace.clear(),
            Key::Return => *model = Model::new(window_width, window_height),
            Key::F11 => app.main_window().set_fullscreen(!app.main_window().is_fullscreen()),
            _ => (),
        }
    });
    
    model.update(update);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);

    draw_model(&draw, model);

    draw.to_frame(app, &frame).unwrap();
}


fn draw_model(draw: &Draw, model: &Model) {
    let origin = pt2(0.0, (model.r1 + model.r2) / 4.0);
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

    for i in 0..model.trace.len() - 1 {
        let alpha = 1.0 - (model.trace.len() - i) as f32 / model.trace.len() as f32;
        draw.line()
            .start(model.trace[i])
            .end(model.trace[i + 1])
            .color(lin_srgba(1.0, 1.0, 1.0, alpha))
            .weight(1.0)
            .z(1.0);
    }
}