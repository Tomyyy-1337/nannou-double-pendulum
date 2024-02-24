use std::collections::VecDeque;

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 700;

pub struct Model {
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

fn model(_app: &App) -> Model {
    Model::new()
}

impl Model {
    pub fn new() -> Self {
        Model {
            r1: (WIDTH / 4) as f32,
            r2: (WIDTH / 4) as f32 - 10.0,
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
        let delta_time = update.since_last.secs() as f32 * 11.0;

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

        let origin = pt2(0.0, (HEIGHT / 2 - 200) as f32);
        let x1 = origin.x + self.r1 * -self.a1.sin() as f32;
        let y1 = origin.y + self.r1 * -self.a1.cos() as f32;
        let x2 = x1 + self.r2 * -self.a2.sin() as f32;
        let y2 = y1 + self.r2 * -self.a2.cos() as f32;
        self.trace.push_back(pt2(x2, y2));
    }
}


fn update(app: &App, model: &mut Model, update: Update) {
    if app.keys.down.contains(&Key::Space) {
        model.trace.clear();
    }
    if app.keys.down.contains(&Key::Return) {
        *model = Model::new();
    }

    model.update(update);

    if model.trace.len() > 5000 {
        model.trace.pop_front();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_model(&draw, model);

    draw.to_frame(app, &frame).unwrap();
}


fn draw_model(draw: &Draw, model: &Model) {
    let origin = pt2(0.0, (HEIGHT / 2 - 200) as f32);
    let (x1, y1) = (
        origin.x + model.r1 * -model.a1.sin(),
        origin.y + model.r1 * -model.a1.cos(),
    );
    let (x2, y2) = (
        x1 + model.r2 * -model.a2.sin(),
        y1 + model.r2 * -model.a2.cos(),
    );
    draw.ellipse().x_y(x1, y1).w_h(model.m1.sqrt() * 2.0, model.m1.sqrt() * 2.0).color(TEAL).z(1.0);
    draw.ellipse().x_y(x2, y2).w_h(model.m2.sqrt() * 2.0, model.m2.sqrt() * 2.0).color(TEAL).z(1.0);
    draw.line().start(origin).end(pt2(x1, y1)).color(TEAL).z(1.0).weight(3.0);
    draw.line().start(pt2(x1, y1)).end(pt2(x2, y2)).color(TEAL).z(1.0).weight(3.0);

    for i in 0..model.trace.len() - 1 {
        draw.line().start(model.trace[i]).end(model.trace[i + 1]).color(lin_srgba(1.0, 1.0, 1.0, 1.0 - (model.trace.len() - i) as f32 / model.trace.len() as f32)).z(0.0).weight(1.0);
    }
}
