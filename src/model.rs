use std::collections::VecDeque;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use egui_plot::{Line, Plot};

pub struct Model {
    pub running: bool,
    pub egui: Egui,
    pub window_width: u32,
    pub window_height: u32,
    pub origin: Point2,
    pub r1: f32,
    pub r2: f32,
    pub m1: f32,
    pub m2: f32,
    pub a1: f32,
    pub a2: f32,
    pub a1_v: f32,
    pub a2_v: f32,
    pub g: f32,
    pub trace: VecDeque<Point2>,
    pub potential_energy_history_inner: VecDeque<f32>,
    pub potential_energy_history_outer: VecDeque<f32>,
    pub kinetic_energy_history_inner: VecDeque<f32>,
    pub kinetic_energy_history_outer: VecDeque<f32>,
    pub frame_count: u64,
}

impl Model {
    pub fn new(window: &std::cell::Ref<'_, Window>, width: u32, height: u32) -> Self {
        Model {
            running: true,
            egui: Egui::from_window(&window),
            window_width: width,
            window_height: height,
            origin: pt2(100.0, 200.0),
            r1: 250.0,
            r2: 240.0,
            m1: 80.0,
            m2: 40.0,
            a1: PI / 2.0 + 0.1,
            a2: PI / 2.0 + 0.2,
            a1_v: 0.0,
            a2_v: 0.0,
            g: 10.0,
            trace: VecDeque::with_capacity(1010),
            potential_energy_history_inner: VecDeque::with_capacity(510),
            potential_energy_history_outer: VecDeque::with_capacity(510),
            kinetic_energy_history_inner: VecDeque::with_capacity(510),
            kinetic_energy_history_outer: VecDeque::with_capacity(510),
            frame_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a1 = PI / 2.0 + 0.1;
        self.a2 = PI / 2.0 + 0.2;
        self.a1_v = 0.0;
        self.a2_v = 0.0;
        self.trace.clear();
    }

    pub fn update_physics(&mut self, update: Update, steps_per_frame: u32) {
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
    
    pub fn upate_trace(&mut self) {
        let x1 = self.r1 * -self.a1.sin() as f32;
        let y1 = self.r1 * -self.a1.cos() as f32;
        let x2 = x1 + self.r2 * -self.a2.sin() as f32;
        let y2 = y1 + self.r2 * -self.a2.cos() as f32;
        
        if self.trace.len() > 1000 {
            self.trace.pop_front();
        }
        if self.frame_count % 2 == 0 {
            self.trace.push_back(pt2(x2, y2));
        }
    }
    
    pub fn update_gui(&mut self, update: Update) {
        self.frame_count += 1;
        if self.kinetic_energy_history_inner.len() > 500 {
            self.kinetic_energy_history_inner.pop_front();
            self.potential_energy_history_outer.pop_front();
            self.kinetic_energy_history_outer.pop_front();
            self.potential_energy_history_inner.pop_front();
        }

        let kinetic_energy_inner = 0.5 * self.m1 * self.r1.powi(2) * self.a1_v.powi(2);
        let kinetic_energy_outer = 0.5 * self.m2 * (self.r1.powi(2) * self.a1_v.powi(2) + self.r2.powi(2) * self.a2_v.powi(2) + 2.0 * self.r1 * self.r2 * self.a1_v * self.a2_v * (self.a1 - self.a2).cos());
        let potential_energy_inner = self.m1 * self.g * self.r1 * (1.0 - self.a1.cos());
        let potential_energy_outer = self.m2 * self.g * (self.r1 * (1.0 - self.a1.cos()) + self.r2 * (1.0 - self.a2.cos()));
        if self.frame_count % 2 == 0 {
            self.kinetic_energy_history_inner.push_back(kinetic_energy_inner);
            self.kinetic_energy_history_outer.push_back(kinetic_energy_outer);
            self.potential_energy_history_inner.push_back(potential_energy_inner);
            self.potential_energy_history_outer.push_back(potential_energy_outer);
        }

        let kinetic_line = Line::new(self.kinetic_energy_history_inner.iter().zip(self.kinetic_energy_history_outer.iter()).enumerate().map(|(i, (a,b))| [(i as u64 + self.frame_count / 2) as f64, (*a + *b) as f64]).collect::<Vec<[f64; 2]>>());
        let potential_line = Line::new(self.potential_energy_history_inner.iter().zip(self.potential_energy_history_outer.iter()).enumerate().map(|(i, (a,b))| [(i as u64 + self.frame_count / 2) as f64, (*a + *b) as f64]).collect::<Vec<[f64; 2]>>());
        let inner_energy_line = Line::new(self.kinetic_energy_history_inner.iter().zip(self.potential_energy_history_inner.iter()).enumerate().map(|(i, (a,b))| [(i as u64 + self.frame_count / 2) as f64, (*a + *b) as f64]).collect::<Vec<[f64; 2]>>());
        let outer_energy_line = Line::new(self.kinetic_energy_history_outer.iter().zip(self.potential_energy_history_outer.iter()).enumerate().map(|(i, (a,b))| [(i as u64 + self.frame_count / 2) as f64, (*a + *b) as f64]).collect::<Vec<[f64; 2]>>());
        let summed_line = Line::new(self.kinetic_energy_history_inner.iter().zip(self.kinetic_energy_history_outer.iter()).zip(self.potential_energy_history_inner.iter().zip(self.potential_energy_history_outer.iter())).enumerate().map(|(i, ((a,b), (c,d)))| [(i as u64 + self.frame_count / 2) as f64, (*a + *b + *c + *d) as f64]).collect::<Vec<[f64; 2]>>());
        let summed_line_2 = Line::new(self.kinetic_energy_history_inner.iter().zip(self.kinetic_energy_history_outer.iter()).zip(self.potential_energy_history_inner.iter().zip(self.potential_energy_history_outer.iter())).enumerate().map(|(i, ((a,b), (c,d)))| [(i as u64 + self.frame_count / 2) as f64, (*a + *b + *c + *d) as f64]).collect::<Vec<[f64; 2]>>());
        
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
            ui.label(format!("Kinetic Energy: {}", kinetic_energy_inner + kinetic_energy_outer));
            ui.label(format!("Potential Energy: {}", potential_energy_inner + potential_energy_outer));
            Plot::new("Kinetic and Potential Energy").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(kinetic_line);
                plot_ui.line(potential_line);
                plot_ui.line(summed_line);
            });
            ui.label(format!("Energy of inner mass: {}", kinetic_energy_inner + potential_energy_inner));
            ui.label(format!("Energy of outer mass: {}", kinetic_energy_outer + potential_energy_outer));
            Plot::new("Energy of inner and outer Mass").view_aspect(2.0).show(ui, |plot_ui| {
                plot_ui.line(inner_energy_line);
                plot_ui.line(outer_energy_line);
                plot_ui.line(summed_line_2);
            });
        });
    }

    pub fn draw(&self, draw: &Draw) {
        let x1 = self.origin.x + self.r1 * -self.a1.sin();
        let y1 = self.origin.y + self.r1 * -self.a1.cos();
        let x2 = x1 + self.r2 * -self.a2.sin();
        let y2 = y1 + self.r2 * -self.a2.cos();
        let color = TEAL;
    
        draw.ellipse()
            .x_y(x1, y1)
            .radius(self.m1.sqrt())
            .color(color)
            .z(3.0);
        draw.ellipse()
            .x_y(x2, y2)
            .radius(self.m2.sqrt())
            .color(color)
            .z(3.0);
        draw.line()
            .start(self.origin)
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
                self.trace
                    .iter()
                    .enumerate()
                    .map(|(i,p)| (pt2(p.x + self.origin.x, p.y + self.origin.y), lin_srgba(1.0, 1.0, 1.0, 1.0 - (self.trace.len() - i) as f32 / self.trace.len() as f32)))
            );
    }
}