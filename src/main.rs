use std::path::{Path};
use eframe::egui;
use eframe::egui::{Image, Vec2};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]).with_app_id("wallpaper-manager"),
        ..Default::default()
    };
    eframe::run_native(
        "WallpaperManager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MainWindow>::default())
        }),
    )
}

struct MainWindow {
    location: String,
    icon_size: f32
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            location: "/home/rose/.steam/steam/steamapps/workshop/content/431960".to_owned(),
            icon_size: 200.0,
        }
    }
}

impl MainWindow {
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
            result.push(path.file_name().unwrap().to_str().unwrap().to_owned());
        }
        if result.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory empty").into());
        }
        Ok(result)
    }

    fn get_column_count(window_width: f32, icon_width: f32) -> i32 {
        (window_width / icon_width) as i32
    }

    fn set_screen_wallpaper(&self, screen: String, ui: &mut egui::Ui, wallpaper: String){
        if ui.button(screen.clone()).clicked() {
            println!("Screen {}: {}", screen.clone(), wallpaper);
            ui.close_menu();
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                })
            });

            egui::containers::ScrollArea::new([false, true]).show(ui, |ui| {
                egui::Grid::new("UniqueId1").show(ui, |ui| {
                    let wallpapers = self.list_wallpapers().unwrap();
                    let mut column = 0;
                    for wallpaper in wallpapers {
                        ui.add(Image::new(self.get_string(wallpaper.clone()).unwrap()).fit_to_exact_size(Vec2::new(self.icon_size, self.icon_size))).context_menu(|ui| {
                            ui.menu_button("Set for screen", |ui| {
                                self.set_screen_wallpaper("DP-1".to_owned(), ui, wallpaper.clone());
                                self.set_screen_wallpaper("HDMI-A-1".to_owned(), ui, wallpaper.clone());
                            });
                            if ui.button("Delete").clicked() {
                                println!("Delete wallpaper: {}", wallpaper);
                                ui.close_menu();
                            }
                        });
                        column += 1;
                        if column == Self::get_column_count(ctx.input(|i: &egui::InputState| i.screen_rect()).width(), self.icon_size) {
                            column = 0;
                            ui.end_row();
                        }
                    }
                });
            });
        });
    }
}
