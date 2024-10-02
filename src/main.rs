#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use core::f64;

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([380.0, 210.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Y+ tool",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    velocity: String,
    density: String,
    viscosity: String,
    length: String,
    yplus: String,
    reynolds: f64,
    y1: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            velocity: "1.0".to_string(),
            density: "1.205".to_string(),
            viscosity: "1.82e-5".to_string(),
            length: "1.0".to_string(),
            yplus: "1.0".to_string(),
            reynolds: f64::NAN,
            y1: f64::NAN,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calulate Y+").highlight();
            egui::Grid::new("Grid")
                .min_col_width(180.0)
                .striped(true)
                .show(ui, |ui| {
                    let name_label = ui.label("Characteristic Velocity: ");
                    ui.text_edit_singleline(&mut self.velocity)
                        .labelled_by(name_label.id);
                    ui.end_row();

                    let name_label = ui.label("Fluid Density: ");
                    ui.text_edit_singleline(&mut self.density)
                        .labelled_by(name_label.id);
                    ui.end_row();

                    let name_label = ui.label("Viscosity: ");
                    ui.text_edit_singleline(&mut self.viscosity)
                        .labelled_by(name_label.id);
                    ui.end_row();

                    let name_label = ui.label("Characteristic Length: ");
                    ui.text_edit_singleline(&mut self.length)
                        .labelled_by(name_label.id);
                    ui.end_row();

                    let name_label = ui.label("Target Y+: ");
                    ui.text_edit_singleline(&mut self.yplus)
                        .labelled_by(name_label.id);
                    ui.end_row();

                    ui.vertical_centered(|ui| {
                        if ui.button("Calculate").clicked() {
                            // 检查并读取输入
                            let velocity: f64 = match self.velocity.parse() {
                                Ok(v) => v,
                                Err(_) => f64::NAN,
                            };
                            let density: f64 = match self.density.parse() {
                                Ok(rho) => rho,
                                Err(_) => f64::NAN,
                            };
                            let viscosity: f64 = match self.viscosity.parse() {
                                Ok(mu) => mu,
                                Err(_) => f64::NAN,
                            };
                            let length: f64 = match self.length.parse() {
                                Ok(l) => l,
                                Err(_) => f64::NAN,
                            };
                            let yplus: f64 = match self.yplus.parse() {
                                Ok(v) => v,
                                Err(_) => f64::NAN,
                            };
                            // 计算
                            self.reynolds = density * velocity * length / viscosity;
                            let cf: f64 = 0.058 * self.reynolds.powf(-0.2);
                            let tw: f64 = 0.5 * cf * density * velocity.powi(2);
                            let ur: f64 = (tw / density).sqrt();
                            self.y1 = yplus * viscosity / ur / density;
                        }
                    });
                    ui.end_row();

                    ui.label("Reynold Number: ");
                    ui.label(format!("{}", self.reynolds));
                    ui.end_row();

                    ui.label("First Layer Height: ");
                    ui.label(format!("{}", self.y1));
                    ui.end_row();
                });
        });
    }
}
