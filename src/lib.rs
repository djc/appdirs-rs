use std::env;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub fn user_data_dir(app: Option<&str>, _: Option<&str>, _: bool) -> PathBuf {
    let mut data_dir = env::home_dir().unwrap();
    data_dir.push("Library/Application Support");
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    data_dir
}

#[cfg(target_os = "macos")]
pub fn site_data_dir(app: Option<&str>, _: Option<&str>) -> PathBuf {
    let mut data_dir = PathBuf::new();
    data_dir.push("/Library/Application Support");
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    data_dir
}

#[cfg(target_os = "macos")]
pub fn user_config_dir(app: Option<&str>, author: Option<&str>, roaming: bool) -> PathBuf {
    user_data_dir(app, author, roaming)
}

#[cfg(target_os = "macos")]
pub fn site_config_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    site_data_dir(app, author)
}

#[cfg(target_os = "macos")]
pub fn user_cache_dir(app: Option<&str>, _: Option<&str>) -> PathBuf {
    let mut cache_dir = env::home_dir().unwrap();
    cache_dir.push("Library/Caches");
    if app.is_some() {
        cache_dir.push(app.unwrap());
    }
    cache_dir
}

#[cfg(target_os = "macos")]
pub fn user_log_dir(app: Option<&str>, _: Option<&str>) -> PathBuf {
    let mut log_dir = env::home_dir().unwrap();
    log_dir.push("Library/Logs");
    if app.is_some() {
        log_dir.push(app.unwrap());
    }
    log_dir
}


#[cfg(test)]
mod tests {

    use std::io::{self, Write};
    use std::path::PathBuf;

    fn to_stderr(name: &str, value: PathBuf) {
        let _ = io::stderr().write(format!("{}: {}\n", name,
                                           value.to_str().unwrap()).as_bytes());
    }

    #[test]
    fn output_dirs() {
        to_stderr("user data dir",
                  super::user_data_dir(Some("AppDirs"), None, false));
        to_stderr("site data dir",
                  super::site_data_dir(Some("AppDirs"), None));
        to_stderr("user config dir",
                  super::user_config_dir(Some("AppDirs"), None, false));
        to_stderr("site config dir",
                  super::site_config_dir(Some("AppDirs"), None));
        to_stderr("user cache dir",
                  super::user_cache_dir(Some("AppDirs"), None));
        to_stderr("user log dir",
                  super::user_log_dir(Some("AppDirs"), None));
    }

}
