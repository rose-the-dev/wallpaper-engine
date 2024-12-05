use std::env::join_paths;
use std::ffi::CString;
use std::fmt::format;
use std::path::{Path, PathBuf};
use eframe::egui;
use eframe::egui::{include_image, Image, Pos2, Widget};
use eframe::egui::menu::menu_button;
use image::ImageReader;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    location: String,
}

impl MyApp {
    fn get_string(&self, wallpaper_dir: String) -> Result<String, std::io::Error> {
        let sub_dir = format!("{base_dir}/{wallpaper_dir}/", base_dir = self.location, wallpaper_dir = wallpaper_dir);
        let paths = std::fs::read_dir(Path::new(sub_dir.as_str()))?;
        for path in paths {
            let path = path?.path();
            let file = path.as_path().file_name().unwrap();
            let name = path.as_path().file_stem().unwrap();
            if name == "preview" {
                return Ok(format!("file://{base_dir}/{wallpaper_dir}/{preview}", base_dir = self.location, wallpaper_dir = wallpaper_dir, preview = file.to_str().unwrap()));
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found").into())
    }

    fn list_wallpapers(&self) -> Result<Vec<String>, std::io::Error> {
        let paths = std::fs::read_dir(Path::new(self.location.as_str()))?;
        let mut result: Vec<String> = Vec::new();
        for path in paths {
            let path = path?.path();
            
        }

        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found"))
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            location: "/home/rose/.steam/steam/steamapps/workshop/content/431960".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ui.close_menu();

                    }
                })
            });

            egui::Grid::new("UniqueId1").show(ui, |ui| {
                ui.image(self.get_string("894376172".to_owned()).unwrap());
                ui.image(self.get_string("894376172".to_owned()).unwrap());
                ui.image(self.get_string("894376172".to_owned()).unwrap());
            });
        });
    }
}
