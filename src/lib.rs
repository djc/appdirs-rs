#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::{site_config_dir, site_data_dir, user_cache_dir,
                user_config_dir, user_data_dir, user_log_dir};

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
mod unix;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub use unix::{site_config_dir, site_data_dir, user_cache_dir,
                user_config_dir, user_data_dir, user_log_dir};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::{site_config_dir, site_data_dir, user_cache_dir,
                user_config_dir, user_data_dir, user_log_dir};


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
                  super::user_data_dir(Some("AppDirs"), Some("djc"), false).unwrap());
        to_stderr("site data dir",
                  super::site_data_dir(Some("AppDirs"), Some("djc")).unwrap());
        to_stderr("user config dir",
                  super::user_config_dir(Some("AppDirs"), Some("djc"), false).unwrap());
        to_stderr("site config dir",
                  super::site_config_dir(Some("AppDirs"), Some("djc")).unwrap());
        to_stderr("user cache dir",
                  super::user_cache_dir(Some("AppDirs"), Some("djc")).unwrap());
        to_stderr("user log dir",
                  super::user_log_dir(Some("AppDirs"), Some("djc")).unwrap());
    }

}
