 use std::collections::HashMap;
use std::path::{Path, PathBuf};
use eframe::egui;
use eframe::egui::{Image, Vec2};
use egui_modal::Modal;
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};

struct Global;
impl Global {
    const CONFIG_DIR: &str = ".config/wallpaper-gui";
    const CONFIG_FILE: &str = "wallpaper.conf";
}

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_app_id("wallpaper-manager").with_inner_size([800.0, 500.0]),
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

#[derive(Serialize, Deserialize)]
struct Config {
    /// Current screen configuration, DP-2, HDMI-A-1, eDP-1, etc
    screens: Vec<String>,
    wallpaper_engine_dir: Option<String>,
    manager_dir: Option<String>,
    silent: bool,
    audio_processing: bool,
    wallpapers: HashMap<String, String>, // screen name, wallpaper id
}
impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            screens: self.screens.clone(),
            wallpaper_engine_dir: self.wallpaper_engine_dir.clone(),
            manager_dir: self.manager_dir.clone(),
            silent: self.silent,
            audio_processing: self.audio_processing,
            wallpapers: self.wallpapers.clone(), // screen name, wallpaper id
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            screens: Vec::new(),
            wallpaper_engine_dir: None,
            manager_dir: None,
            silent: false,
            audio_processing: true,
            wallpapers: HashMap::new(),
        }
    }
}

fn read_config(config_file: String) -> Config {
    let config_data = std::fs::read_to_string(config_file).expect("Error reading config file");
    serde_json::from_str(config_data.as_str()).unwrap()
}

fn write_config(config_file: String, config: Config) {
    std::fs::write(config_file, serde_json::to_string(&config).expect("Error serializing config file")).expect("Error writing config file");
}

struct WallpaperInfo {
    /// Id of wallpaper (directory without full path of other files)
    id: String,
    /// Full path of wallpaper files with id.
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
    pub fn new(path: PathBuf) -> core::result::Result<Self, std::io::Error> {
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
    config: Config,
    page: i32,
    //location: String,
    icon_size: f32,
    wallpaper: Option<WallpaperInfo>,
}

impl Default for MainWindow {
    fn default() -> Self {
        let binding = std::env::home_dir().unwrap();
        let config_dir = format!("{0}/{1}", binding.to_str().unwrap(), Global::CONFIG_DIR);
        let config_file = format!("{0}/{1}", config_dir, Global::CONFIG_FILE);
        if std::fs::exists(config_dir.clone()).is_ok_and(|x| x == false) {
            std::fs::create_dir(config_dir).expect("Unable to create config dir");
        }
        if std::fs::exists(config_file.clone()).is_ok_and(|x| x == false) {
            let conf = Config::default();
            write_config(config_file.clone(), conf);
        }
        Self {
            config: read_config(config_file),
            page: 1,
            //location: "/home/rose/.steam/steam/steamapps/workshop/content/431960".to_owned(),
            icon_size: 200.0,
            wallpaper: None,
        }
    }
}

impl MainWindow {
    fn get_wallpapers(&self) -> core::result::Result<Vec<WallpaperInfo>, std::io::Error> {
        let paths = std::fs::read_dir(Path::new(self.config.wallpaper_engine_dir.clone().unwrap().as_str()))?; // WRONG
        let mut result: Vec<WallpaperInfo> = Vec::new();
        for path in paths {
            let path = path?.path();
            //result.push(path.file_name().unwrap().to_str().unwrap().to_owned());
            result.push(WallpaperInfo::new(path)?);
        }
        if result.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory empty").into());
        }
        Ok(result)
    }

    fn get_column_count(window_width: f32, icon_width: f32) -> i32 {
        (window_width / icon_width) as i32
    }

    fn set_screen_wallpaper(&mut self, screen: String, ui: &mut egui::Ui, wallpaper: String){
        if ui.button(screen.clone()).clicked() {
            //println!("Screen {}: {}", screen.clone(), wallpaper);

            //let mut config = read_config("".to_string());
            let mut config = self.config.clone();
            //let mut config = config.screens;
            //*config.get_mut(&screen) = wallpaper;
            //let mut config = read_config(self.config_file.clone());
            *config.wallpapers.get_mut(&screen).unwrap() = wallpaper;
            //write_config(self.config_file.clone(), config);
            self.config = config;

            std::process::Command::new("systemctl").arg("--user").arg("restart").arg("wallpaperengine.service")
                .output().expect("Failed to execute systemctl command");
            ui.close_menu();
        }
    }

    fn delete_wallpaper(wallpaper: WallpaperInfo) {
        println!("Deleting wallpaper: {}", wallpaper.full_path.to_str().unwrap());
        std::fs::remove_dir_all(wallpaper.full_path).unwrap()
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let delete_modal = Modal::new(ctx, "delete_modal");
        delete_modal.show(|ui| {
            delete_modal.title(ui, "Delete wallpaper?");
            delete_modal.frame(ui, |ui| {
                delete_modal.body(ui, "Are you sure you want to permanently delete this wallpaper?");
            });
            delete_modal.buttons(ui, |ui| {
                delete_modal.button(ui, "Close");
                if delete_modal.button(ui, "Delete").clicked() {
                    Self::delete_wallpaper(self.wallpaper.clone().unwrap())
                };
            });
        });

        let about_modal = Modal::new(ctx, "about_modal");
        about_modal.show(|ui| {
            about_modal.title(ui, "About wallpaper manager");
            about_modal.frame(ui, |ui| {
                about_modal.body(ui, "wallpaper manager to be used with linux-wallpaperengine");
            });
            about_modal.buttons(ui, |ui| {
                about_modal.button(ui, "Close");
            });
        });

        //let wallpapers_modal = Modal::new(ctx, "wallpapers_modal");
        //wallpapers_modal.show(|ui| {//
        //});

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Get wallpapers").clicked() {
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        about_modal.open();
                        ui.close_menu();
                    }
                });
            });

            match self.page {
                0 => {

                },
                1 => {
                    egui::containers::ScrollArea::new([false, true]).show(ui, |ui| {
                        egui::Grid::new("WallpaperGrid").show(ui, |ui| {
                            let wallpapers = self.get_wallpapers().unwrap();
                            let mut column = 0;
                            for wallpaper in wallpapers {
                                ui.add(Image::new(format!("file://{}", wallpaper.preview_file)).fit_to_exact_size(Vec2::new(self.icon_size, self.icon_size))).context_menu(|ui| {
                                    ui.menu_button("Set for screen", |ui| {
                                        let mons = display_info::DisplayInfo::all().unwrap();
                                        for mon in mons {
                                            self.set_screen_wallpaper(mon.name, ui, wallpaper.id.clone());
                                        }
                                    });
                                    if ui.button("Delete").clicked() {
                                        self.wallpaper = Some(wallpaper);
                                        delete_modal.open();
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
                },
                _ => {

                }
            };
        });
    }
}