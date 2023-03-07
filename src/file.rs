use std::{env, fs};
use std::path::{Path, PathBuf};

fn get_current_directory() -> PathBuf {
    return env::current_exe().unwrap();
}

pub fn read_config(file_name: String) -> Option<String> {
    read_config_in_home_dir(&file_name).or(read_config_from_config_dir_under_home_dir(&file_name)).or(read_config_in_exe_dir(&file_name))
}

fn read_config_in_home_dir(file_name: &String) -> Option<String> {
    let config_path = Path::new(&env::var("HOME").unwrap()).join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(fs::read_to_string(config_path).expect("unable to read config file"))
    } else {
        None
    }
}

fn read_config_from_config_dir_under_home_dir(file_name: &String) -> Option<String> {
    let config_path = Path::new(&env::var("HOME").unwrap()).join(".config").join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(fs::read_to_string(config_path).expect("unable to read config file"))
    } else {
        None
    }
}

fn read_config_in_exe_dir(file_name: &String) -> Option<String> {
    let config_path = get_current_directory().parent().unwrap().join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(fs::read_to_string(config_path).expect("unable to read config file"))
    } else {
        None
    }
}
