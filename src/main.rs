use std::collections::VecDeque;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use egui_plot::{Line, Plot};

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

pub struct Model {
    running: bool,
    egui: Egui,
    window_width: u32,
    window_height: u32,
    origin: Point2,
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
    potential_energy_history: VecDeque<f32>,
    kinetic_energy_history: VecDeque<f32>,
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

impl Model {
    pub fn new(window: &std::cell::Ref<'_, Window>, width: u32, height: u32) -> Self {
        Model {
            running: true,
            egui: Egui::from_window(&window),
            window_width: width,
            window_height: height,
            origin: pt2(0.0, 200.0),
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
            potential_energy_history: VecDeque::with_capacity(2010),
            kinetic_energy_history: VecDeque::with_capacity(2010),
        }
    }

    pub fn reset(&mut self) {
        self.a1 = PI / 2.0 + 0.1;
        self.a2 = PI / 2.0 + 0.2;
        self.a1_v = 0.0;
        self.a2_v = 0.0;
        self.trace.clear();
    }

    fn update_physics(&mut self, update: Update, steps_per_frame: u32) {
        for _ in 0..steps_per_frame {
            let delta_time = update.since_last.secs() as f32 * 10.0 / steps_per_frame as f32;
    
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
        }
    }

    fn upate_trace(&mut self) {
        let x1 = self.origin.x + self.r1 * -self.a1.sin() as f32;
        let y1 = self.origin.y + self.r1 * -self.a1.cos() as f32;
        let x2 = x1 + self.r2 * -self.a2.sin() as f32;
        let y2 = y1 + self.r2 * -self.a2.cos() as f32;

        if self.trace.len() > 5000 {
            self.trace.pop_front();
        }
        self.trace.push_back(pt2(x2, y2));
    }

    fn update_gui(&mut self, update: Update) {
        if self.kinetic_energy_history.len() >= 2000 {
            self.kinetic_energy_history.pop_front();
        }
        if self.potential_energy_history.len() >= 2000 {
            self.potential_energy_history.pop_front();
        }
        let kinetic_energy = 0.5 * self.m1 * self.r1.powi(2) * self.a1_v.powi(2) + 0.5 * self.m2 * (self.r1.powi(2) * self.a1_v.powi(2) + self.r2.powi(2) * self.a2_v.powi(2) + 2.0 * self.r1 * self.r2 * self.a1_v * self.a2_v * (self.a1 - self.a2).cos());
        let potential_energy = self.m1 * self.g * self.r1 * (1.0 - self.a1.cos()) + self.m2 * self.g * (self.r1 * (1.0 - self.a1.cos()) + self.r2 * (1.0 - self.a2.cos()));
        let total_energy = kinetic_energy + potential_energy;

        self.kinetic_energy_history.push_back(kinetic_energy);
        self.potential_energy_history.push_back(potential_energy);

        let kinetic_line = Line::new(self.kinetic_energy_history.iter().enumerate().map(|(i, y)| {
            let x = i as f64 * 0.01;
            [x, *y as f64]
        }).collect::<Vec<[f64; 2]>>());
        let potential_line = Line::new(self.potential_energy_history.iter().enumerate().map(|(i, y)| {
            let x = i as f64 * 0.01;
            [x, *y as f64]
        }).collect::<Vec<[f64; 2]>>());
        let summed_line = Line::new(self.kinetic_energy_history.iter().enumerate().zip(self.potential_energy_history.iter()).map(|((i, y1), y2)| {
            let x = i as f64 * 0.01;
            [x, (*y1 + *y2) as f64]
        }).collect::<Vec<[f64; 2]>>());

        self.egui.set_elapsed_time(update.since_start);
        let ctx = self.egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Länge des inneren Pendels:");
            ui.add(egui::Slider::new(&mut self.r1, 100.0..=500.0));
            ui.label("Länge des äußeren Pendels:");
            ui.add(egui::Slider::new(&mut self.r2, 100.0..=500.0));
            ui.label("Inneres Gewicht:");
            ui.add(egui::Slider::new(&mut self.m1, 10.0..=100.0));
            ui.label("Äußeres Gewicht:");
            ui.add(egui::Slider::new(&mut self.m2, 10.0..=100.0));
            ui.label("Origin x:");
            ui.add(egui::Slider::new(&mut self.origin.x, -(self.window_width as f32) / 2.0..=self.window_width as f32 / 2.0));
            ui.label("Origin y:");
            ui.add(egui::Slider::new(&mut self.origin.y, -(self.window_height as f32) / 2.0..=self.window_height as f32 / 2.0));
        });    
        egui::Window::new("Plot").show(&ctx, |ui|{
            ui.label(format!("Kinetic Energy: {:.2}, Potential Energy: {:.2}", kinetic_energy, potential_energy));
            ui.label(format!("Total Energy: {:.2}", total_energy));
            Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(kinetic_line);
                plot_ui.line(potential_line);
                plot_ui.line(summed_line);
            })
        });
    }
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
        .points_colored(
            model.trace
                .iter()
                .enumerate()
                .map(|(i,p)| (*p, lin_srgba(1.0, 1.0, 1.0, 1.0 - (model.trace.len() - i) as f32 / model.trace.len() as f32)))
        )
        .z(1.0);

}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}