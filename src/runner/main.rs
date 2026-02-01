use std::collections::HashMap;
use ctrlc::set_handler;
use serde::{Deserialize, Serialize};

struct Global;
impl Global {
    const CONFIG_DIR: &str = ".config/wallpaper-gui";
    const CONFIG_FILE: &str = "wallpaper.conf";
}

#[derive(Serialize, Deserialize)]
struct Config {
    silent: bool,
    no_audio_processing: bool,
    wallpapers: HashMap<String, String>, // screen name, wallpaper id
}
#[derive(Serialize, Deserialize)]
struct ScreenInfo {
    screen_name: String,
    wallpaper_id: String,
}

fn main() {
    let binding = std::env::home_dir().unwrap();
    //let config_dir = format!("{0}/{1}", binding.to_str().unwrap(), Global::CONFIG_DIR);
    let config_file = format!("{0}/{1}/{2}", binding.to_str().unwrap(), Global::CONFIG_DIR, Global::CONFIG_FILE);
    let mut config_file = config_file.as_str();


    let args: Vec<String> = std::env::args().collect();

    let mut arg_num = 0;
    for i in 1..args.len() {
        let arg = &args[i];
        println!("{}", arg);
        let pair = arg.split('=').collect::<Vec<&str>>();
        if pair.len() != 2 {
            println!("Error with argument {num}, {arg}.", num=arg_num, arg=arg);
            panic!("Error with argument {num}, {arg}", num=arg_num, arg=arg);
        }
        if pair[0] == "--config" {
            config_file = pair[1];
        }
        else {
            println!("Argument {num} unknown; {arg}", num=arg_num, arg=arg);
            panic!("Argument {num} unknown; {arg}", num=arg_num, arg=arg);
        }
        arg_num += 1;
    }
    if !std::fs::exists(config_file).unwrap(){
        println!("Error: config file {} does not exist!", config_file);
        panic!("Config file {} does not exist!", config_file);
    }
    let config_data = std::fs::read_to_string(&config_file)
        .expect("Error reading config file");
    let config: Config = serde_json::from_str(config_data.as_str()).unwrap();
    let mut proc = std::process::Command::new("linux-wallpaperengine");
    proc.env("XDG_SESSION_TYPE", "wayland");
    if config.silent {
        proc.arg("--silent");
    }
    if config.no_audio_processing {
        proc.arg("--no-audio-processing");
    }
    for screen in config.wallpapers {
        proc.arg("--screen-root").arg(screen.0).arg("--bg").arg(screen.1);
    }
    let mut child = proc.spawn().expect("failed to execute process");
    let mut wait = true;
    set_handler(move || {
        println!("Killed process");
        child.kill().unwrap();
        wait = false;
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    while wait { }
}
