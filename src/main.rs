#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::{epaint::FontFamily, Context, FontData, FontDefinitions, IconData};
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use std::{f64, fs::read, sync::Arc};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let icon_data = include_bytes!("../assets/y_letter.ico");
    let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Ico).unwrap();
    let rgba_data = img.into_rgba8();
    let (w, h) = (rgba_data.width(), rgba_data.height());
    let raw_data: Vec<u8> = rgba_data.into_raw();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(Arc::<IconData>::new(IconData {
                rgba: raw_data,
                width: w,
                height: h,
            }))
            .with_inner_size([380.0, 220.0]),
        centered: true,
        persist_window: true,
        ..Default::default()
    };
    eframe::run_native(
        "Y+工具",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

// fn load_fonts(ctx: &egui::Context) {
//     let mut fonts = egui::FontDefinitions::default();
//     fonts.font_data.insert(
//         "my_font".to_owned(),
//         egui::FontData::from_static(include_bytes!("../assets/msyh.ttc")),
//     );
//     fonts
//         .families
//         .get_mut(&egui::FontFamily::Proportional)
//         .unwrap()
//         .insert(0, "my_font".to_owned());
//     fonts
//         .families
//         .get_mut(&egui::FontFamily::Monospace)
//         .unwrap()
//         .push("my_font".to_owned());
//     ctx.set_fonts(fonts);
// }

// fn print_system_fonts() {
//     for name in SystemSource::new().all_fonts().unwrap() {
//         println!("{:?}", name.load().unwrap().postscript_name().unwrap());
//     }
// }

fn load_system_font(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    let handle = match SystemSource::new().select_by_postscript_name("MicrosoftYaHeiRegular") {
        Ok(v) => v,
        Err(_) => SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .unwrap(),
    };

    let buf: Vec<u8> = match handle {
        Handle::Memory { bytes, .. } => bytes.to_vec(),
        Handle::Path { path, .. } => read(path).unwrap(),
    };

    const FONT_SYSTEM_SANS_SERIF: &'static str = "System Sans Serif";

    fonts
        .font_data
        .insert(FONT_SYSTEM_SANS_SERIF.to_owned(), FontData::from_owned(buf));

    if let Some(vec) = fonts.families.get_mut(&FontFamily::Proportional) {
        vec.push(FONT_SYSTEM_SANS_SERIF.to_owned());
    }

    if let Some(vec) = fonts.families.get_mut(&FontFamily::Monospace) {
        vec.push(FONT_SYSTEM_SANS_SERIF.to_owned());
    }

    ctx.set_fonts(fonts);
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
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // load_fonts(&cc.egui_ctx);
        load_system_font(&cc.egui_ctx);
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
            ui.heading("计算第一层网格高度").highlight();
            egui::Grid::new("Grid")
                .min_col_width(180.0)
                .striped(true)
                .show(ui, |ui| {
                    let name_label = ui.label("特征速度: ");
                    ui.text_edit_singleline(&mut self.velocity)
                        .labelled_by(name_label.id)
                        .on_hover_text("m/s");
                    ui.end_row();

                    let name_label = ui.label("流体密度: ");
                    ui.text_edit_singleline(&mut self.density)
                        .labelled_by(name_label.id)
                        .on_hover_text("kg/m^3");
                    ui.end_row();

                    let name_label = ui.label("动力粘度系数: ");
                    ui.text_edit_singleline(&mut self.viscosity)
                        .labelled_by(name_label.id)
                        .on_hover_text("N·s/m^2");
                    ui.end_row();

                    let name_label = ui.label("特征长度: ");
                    ui.text_edit_singleline(&mut self.length)
                        .labelled_by(name_label.id)
                        .on_hover_text("m");
                    ui.end_row();

                    let name_label = ui.label("目标Y+: ");
                    ui.text_edit_singleline(&mut self.yplus)
                        .labelled_by(name_label.id);
                    ui.end_row();

                    ui.vertical_centered(|ui| {
                        if ui.button("计算").clicked() {
                            // 检查并读取输入
                            let velocity: f64 = match self.velocity.parse() {
                                Ok(v) => {
                                    if v >= 0. {
                                        v
                                    } else {
                                        f64::NAN
                                    }
                                }
                                Err(_) => f64::NAN,
                            };
                            let density: f64 = match self.density.parse() {
                                Ok(rho) => {
                                    if rho > 0. {
                                        rho
                                    } else {
                                        f64::NAN
                                    }
                                }
                                Err(_) => f64::NAN,
                            };
                            let viscosity: f64 = match self.viscosity.parse() {
                                Ok(mu) => {
                                    if mu > 0. {
                                        mu
                                    } else {
                                        f64::NAN
                                    }
                                }
                                Err(_) => f64::NAN,
                            };
                            let length: f64 = match self.length.parse() {
                                Ok(l) => {
                                    if l > 0. {
                                        l
                                    } else {
                                        f64::NAN
                                    }
                                }
                                Err(_) => f64::NAN,
                            };
                            let yplus: f64 = match self.yplus.parse() {
                                Ok(v) => {
                                    if v > 0. {
                                        v
                                    } else {
                                        f64::NAN
                                    }
                                }
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

                    ui.label("雷诺数: ");
                    ui.label(format!("{}", self.reynolds));
                    ui.end_row();

                    ui.label("第一层网格高度: ");
                    ui.label(format!("{}", self.y1)).on_hover_text("m");
                    ui.end_row();
                });
        });
    }
}
