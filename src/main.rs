use std::fmt::format;
use std::path::{Path, PathBuf};
use eframe::egui;
use eframe::egui::{Image, Vec2};

fn main() -> eframe::Result {
    println!("Starting");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_app_id("wallpaper-manager").with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    println!("Running");
    eframe::run_native(
        "WallpaperManager",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MainWindow>::default())
        }),
    )
}

struct WallpaperInfo {
    /// Id of wallpaper (directory without full path of other files)
    id: String,
    full_path: PathBuf,
    /// Full path to preview file.
    preview_file: String,

}

impl Clone for WallpaperInfo {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            full_path: self.full_path.clone(),
            preview_file: self.preview_file.clone(),
        }
    }
}

impl WallpaperInfo {
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let id = path.as_path().file_name().unwrap().to_str().unwrap().to_owned();
        let paths = std::fs::read_dir(Path::new(path.as_path()))?;
        for path2 in paths {
            let path2 = path2?.path();
            //let file = path2.as_path().file_name().unwrap();
            let name = path2.as_path().file_stem().unwrap();
            if name == "preview" {
                return Ok(Self {
                    id: id.clone(),
                    full_path: path.clone(),
                    preview_file: path2.as_path().to_str().unwrap().to_owned(),
                });
            }
        }
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"))
    }
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
    fn get_wallpapers(&self) -> Result<Vec<WallpaperInfo>, std::io::Error> {
        let paths = std::fs::read_dir(Path::new(self.location.as_str()))?;
        let mut result: Vec<WallpaperInfo> = Vec::new();
        for path in paths {
            let path = path?.path();
            //result.push(path.file_name().unwrap().to_str().unwrap().to_owned());
            result.push(WallpaperInfo::new(path).unwrap());
        }
        if result.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory empty").into());
        }
        Ok(result)
    }

    fn get_column_count(window_width: f32, icon_width: f32) -> i32 {
        (window_width / icon_width) as i32
    }

    fn set_screen_wallpaper(screen: String, ui: &mut egui::Ui, wallpaper: String){
        if ui.button(screen.clone()).clicked() {
            println!("Screen {}: {}", screen.clone(), wallpaper);
            ui.close_menu();
        }
    }

    fn delete_wallpaper(wallpaper: WallpaperInfo) {
        println!("Deleting wallpaper: {}", wallpaper.full_path.to_str().unwrap());
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
                egui::Grid::new("WallpaperGrid").show(ui, |ui| {
                    let wallpapers = self.get_wallpapers().unwrap();
                    let mut column = 0;
                    for wallpaper in wallpapers {
                        ui.add(Image::new(format!("file://{}", wallpaper.preview_file)).fit_to_exact_size(Vec2::new(self.icon_size, self.icon_size))).context_menu(|ui| {
                            ui.menu_button("Set for screen", |ui| {
                                let mons = display_info::DisplayInfo::all().unwrap();
                                for mon in mons {
                                    Self::set_screen_wallpaper(mon.name, ui, wallpaper.id.clone());
                                }
                            });
                            if ui.button("Delete").clicked() {
                                Self::delete_wallpaper(wallpaper.clone());
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
