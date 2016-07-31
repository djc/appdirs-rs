use std::env;
use std::path::PathBuf;

fn home_dir_relative(rel: &str, app: Option<&str>) -> Result<PathBuf, ()> {
    let data_dir_res = env::home_dir();
    if data_dir_res.is_none() {
        return Err(());
    }
    let mut data_dir = data_dir_res.unwrap();
    data_dir.push(rel);
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    Ok(data_dir)
}

pub fn user_data_dir(app: Option<&str>, _: Option<&str>, _: bool) -> Result<PathBuf, ()> {
    home_dir_relative("Library/Application Support", app)
}

pub fn site_data_dir(app: Option<&str>, _: Option<&str>) -> Result<PathBuf, ()> {
    let mut data_dir = PathBuf::new();
    data_dir.push("/Library/Application Support");
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    Ok(data_dir)
}

pub fn user_config_dir(app: Option<&str>, author: Option<&str>, roaming: bool)
                       -> Result<PathBuf, ()> {
    user_data_dir(app, author, roaming)
}

pub fn site_config_dir(app: Option<&str>, author: Option<&str>) -> Result<PathBuf, ()> {
    site_data_dir(app, author)
}

pub fn user_cache_dir(app: Option<&str>, _: Option<&str>) -> Result<PathBuf, ()> {
    home_dir_relative("Library/Caches", app)
}

pub fn user_log_dir(app: Option<&str>, _: Option<&str>) -> Result<PathBuf, ()> {
    home_dir_relative("Library/Logs", app)
}
