use std::collections::VecDeque;
use nannou::prelude::*;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct Pendumlum {
    pub origin: Point2,
    pub r1: f64,
    pub r2: f64,
    pub m1: f64,
    pub m2: f64,
    pub a1: f64,
    pub a2: f64,
    pub a1_v: f64,
    pub a2_v: f64,
    pub g: f64,
    pub trace: VecDeque<Point2>,
    pub color_r: f64,
    pub color_g: f64,
    pub color_b: f64,
    offset: f64,
}

impl Pendumlum {
    pub fn new(r: f64, g: f64, b: f64, offset: f64) -> Pendumlum {
        Pendumlum {
            origin: pt2(0.0, 0.0),
            r1: 200.0,
            r2: 300.0,
            m1: 200.0,
            m2: 200.0,
            a1: PI_F64 + 1.0,
            a2: PI_F64 + 1.0 + offset,
            a1_v: 0.0,
            a2_v: 0.0,
            g: 4.0,
            trace: VecDeque::new(),
            color_r: r,
            color_b: b,
            color_g: g,
            offset,
        }
    }

    pub fn reset(&mut self, r1: f64, r2: f64, m1: f64, m2: f64, a1: f64, a2: f64) {
        self.r1 = r1;
        self.r2 = r2;
        self.m1 = m1;
        self.m2 = m2;

        self.a1 = a1;
        self.a2 = a2 + self.offset;
        self.a1_v = 0.0;
        self.a2_v = 0.0;
        self.trace.clear();
    }

    pub fn update_physics(&mut self, update: Update, steps_per_frame: u32) {
        for _ in 0..steps_per_frame {
            let delta_time = update.since_last.secs() as f64 * 10.0 / steps_per_frame as f64;
    
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
        let x1 = self.r1 * -self.a1.sin() as f64;
        let y1 = self.r1 * -self.a1.cos() as f64;
        let x2 = x1 + self.r2 * -self.a2.sin() as f64;
        let y2 = y1 + self.r2 * -self.a2.cos() as f64;
        
        if self.trace.len() > 2 {
            self.trace.pop_front();
        }
        self.trace.push_back(pt2(x2 as f32, y2 as f32));

    }

    pub fn draw(&self, draw: &Draw) { 
        if self.trace.len() < 2 {
            return;
        }
        draw.line()
            .weight(1.0)
            .z(1.0)
            .color(lin_srgba(self.color_r, self.color_g, self.color_b, 0.07))
            .start(self.trace[0])
            .end(self.trace[1]);
    }



}

pub struct Model {
    pub running: bool,
    pub window_width: u32,
    pub window_height: u32,
    pub frame_count: u64,
    pub num_pendulums: u32,
    pub pendulums: Vec<Pendumlum>,
    pub reset_timer: u64,
}

impl Model {
    pub fn new(width: u32, height: u32) -> Self {
        let num_pendulums = 7000;
        Model {
            num_pendulums,
            running: true,
            window_width: width,
            window_height: height,
            frame_count: 0,
            reset_timer: 0,
            pendulums: (1..=num_pendulums).map(|i| {
                let hue = (i as f64 / num_pendulums as f64) * 2.0 * PI_F64;  
                let r = hue.sin() * 0.5 + 0.5;
                let g = hue.cos() * 0.5 + 0.5;
                let b = (hue + PI_F64 / 2.0).cos() * 0.5 + 0.5;
                let offset = (i as f64 / num_pendulums as f64) * 0.001;
                Pendumlum::new(r,g,b, offset)
            }).collect(),
        }
    }

    pub fn chaos_factor(&self) -> f64 {
        let first_a1 = self.pendulums[0].a1;
        let last_a1 = self.pendulums[self.num_pendulums as usize - 1].a1;
        let delta = (last_a1 - first_a1).abs();
        delta / (PI_F64)
    }

    pub fn reset(&mut self) {
        let r1 = random_range(100.0, 400.0);
        let r2 = 500.0 - r1;

        let m1 = random_range(100.0, 400.0);
        let m2 = random_range(100.0, 400.0);

        let a1 = PI_F64 + random_range(-1.5, 1.5);
        let a2 = PI_F64 + random_range(-1.5, 1.5);
        
        self.frame_count = 0;
        self.pendulums.iter_mut().for_each(|p| p.reset(r1,r2,m1,m2,a1,a2));
        self.reset_timer = 0;
    }

    pub fn clear_trace(&mut self) {
        self.frame_count = 0;
        self.pendulums.iter_mut().for_each(|p| p.trace.clear());
    }

    pub fn update_physics(&mut self, update: Update, steps_per_frame: u32) {
        self.pendulums.par_iter_mut().for_each(|p| {
            p.update_physics(update, steps_per_frame);
            p.upate_trace();
        });
    }

    pub fn draw(&self, draw: &Draw) {
        self.pendulums.iter().for_each(|p| p.draw(draw));
    }        
}