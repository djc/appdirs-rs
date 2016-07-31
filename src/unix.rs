use std::env;
use std::path::PathBuf;

fn home_dir_relative(xdg_key: &str, rel: &str, app: Option<&str>) -> Result<PathBuf, ()> {
    let mut data_dir = PathBuf::new();
    match env::var_os(xdg_key) {
        Some(dir) => { data_dir.push(dir); },
        None => {
            let home_res = env::home_dir();
            if home_res.is_none() {
                return Err(());
            }
            data_dir.push(home_res.unwrap());
            data_dir.push(rel);
        },
    };
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    Ok(data_dir)
}

pub fn user_data_dir(app: Option<&str>, _: Option<&str>, _: bool) -> Result<PathBuf, ()> {
    home_dir_relative("XDG_DATA_HOME", ".local/share", app)
}

pub fn site_data_dir(app: Option<&str>, _: Option<&str>) -> Result<PathBuf, ()> {
    let mut data_dir = PathBuf::new();
    let default = "/usr/local/share";
    match env::var_os("XDG_DATA_DIRS") {
        Some(joined) => {
            let first = env::split_paths(&joined).next();
            match first {
                Some(dir) => { data_dir.push(dir); },
                None => { data_dir.push(default); },
            };
        },
        None => { data_dir.push(default); },
    };
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    Ok(data_dir)
}

pub fn user_config_dir(app: Option<&str>, _: Option<&str>, _: bool) -> Result<PathBuf, ()> {
    home_dir_relative("XDG_CONFIG_HOME", ".config", app)
}

pub fn site_config_dir(app: Option<&str>, _: Option<&str>) -> Result<PathBuf, ()> {
    let mut data_dir = PathBuf::new();
    let default = "/etc/xdg";
    match env::var_os("XDG_CONFIG_DIRS") {
        Some(joined) => {
            let first = env::split_paths(&joined).next();
            match first {
                Some(dir) => { data_dir.push(dir); },
                None => { data_dir.push(default); },
            };
        },
        None => { data_dir.push(default); },
    };
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    Ok(data_dir)
}

pub fn user_cache_dir(app: Option<&str>, _: Option<&str>) -> Result<PathBuf, ()> {
    home_dir_relative("XDG_CACHE_HOME", ".cache", app)
}

pub fn user_log_dir(app: Option<&str>, author: Option<&str>) -> Result<PathBuf, ()> {
    let log_dir = user_cache_dir(app, author);
    match log_dir {
        Ok(mut log_dir) => { log_dir.push("log"); Ok(log_dir) },
        Err(err) => Err(err),
    }
}
