use dirs::home_dir;
use std::path::PathBuf;
use std::{env, fs};

fn get_current_directory() -> PathBuf {
    return env::current_exe().unwrap();
}

pub fn get_file_from_config_dir(file_name: String) -> Option<PathBuf> {
    read_config_in_home_dir(&file_name)
        .or(read_config_from_gptr_dir_under_config_under_home_dir(
            &file_name,
        ))
        .or(read_config_from_config_dir_under_home_dir(&file_name))
        .or(read_config_in_exe_dir(&file_name))
}

pub fn read_config(file_name: String) -> Option<String> {
    get_file_from_config_dir(file_name)
        .and_then(|file| fs::read_to_string(file).ok())
        .or_else(|| None)
}

fn read_config_in_home_dir(file_name: &String) -> Option<PathBuf> {
    let config_path = home_dir().unwrap().join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(config_path)
    } else {
        None
    }
}

fn read_config_from_config_dir_under_home_dir(file_name: &String) -> Option<PathBuf> {
    let config_path = home_dir().unwrap().join(".config").join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(config_path)
    } else {
        None
    }
}

fn read_config_in_exe_dir(file_name: &String) -> Option<PathBuf> {
    let config_path = get_current_directory().parent().unwrap().join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(config_path)
    } else {
        None
    }
}

/**
 * Read config from ~/.config/gptr
 */
fn read_config_from_gptr_dir_under_config_under_home_dir(file_name: &String) -> Option<PathBuf> {
    let config_path = home_dir().unwrap().join(".config/gptr").join(file_name);
    if config_path.exists() && config_path.is_file() {
        Some(config_path)
    } else {
        None
    }
}
