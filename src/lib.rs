#[cfg(target_os = "windows")]
extern crate shell32;
#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate ole32;

#[cfg(not(target_os = "windows"))]
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

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn user_data_dir(app: Option<&str>, _: Option<&str>, _: bool) -> PathBuf {
    let mut data_dir = PathBuf::new();
    match env::var_os("XDG_DATA_HOME") {
        Some(dir) => { data_dir.push(dir); },
        None => {
            data_dir.push(env::home_dir().unwrap());
            data_dir.push(".local/share");
        },
    };
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

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn site_data_dir(app: Option<&str>, _: Option<&str>) -> PathBuf {
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
    data_dir
}

#[cfg(target_os = "macos")]
pub fn user_config_dir(app: Option<&str>, author: Option<&str>, roaming: bool) -> PathBuf {
    user_data_dir(app, author, roaming)
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn user_config_dir(app: Option<&str>, _: Option<&str>, _: bool) -> PathBuf {
    let mut config_dir = PathBuf::new();
    match env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => { config_dir.push(dir); },
        None => {
            config_dir.push(env::home_dir().unwrap());
            config_dir.push(".config");
        },
    };
    if app.is_some() {
        config_dir.push(app.unwrap());
    }
    config_dir
}

#[cfg(target_os = "macos")]
pub fn site_config_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    site_data_dir(app, author)
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn site_config_dir(app: Option<&str>, _: Option<&str>) -> PathBuf {
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
    data_dir
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

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn user_cache_dir(app: Option<&str>, _: Option<&str>) -> PathBuf {
    let mut cache_dir = PathBuf::new();
    match env::var_os("XDG_CACHE_HOME") {
        Some(dir) => { cache_dir.push(dir); },
        None => {
            cache_dir.push(env::home_dir().unwrap());
            cache_dir.push(".cache");
        },
    };
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

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn user_log_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    let mut log_dir = user_cache_dir(app, author);
    log_dir.push("log");
    log_dir
}

#[cfg(target_os = "windows")]
static APPDATA_GUID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 1052149211,
    Data2: 26105,
    Data3: 19702,
    Data4: [160, 58, 227, 239, 101, 114, 159, 61],
};

#[cfg(target_os = "windows")]
static COMMON_APPDATA_GUID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 1655397762,
    Data2: 64961,
    Data3: 19907,
    Data4: [169, 221, 7, 13, 29, 73, 93, 151],
};

#[cfg(target_os = "windows")]
static LOCAL_APPDATA_GUID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 4055050117,
    Data2: 28602,
    Data3: 20431,
    Data4: [157, 85, 123, 142, 127, 21, 112, 145],
};

#[cfg(target_os = "windows")]
use std::ptr;

#[cfg(target_os = "windows")]
use std::ffi::OsString;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;

#[cfg(target_os = "windows")]
use std::slice;

#[cfg(target_os = "windows")]
fn get_dir(id: &winapi::shtypes::KNOWNFOLDERID) -> Result<OsString, ()> {
    let mut result: winapi::PWSTR = ptr::null_mut();
    let error;
    unsafe {
        error = shell32::SHGetKnownFolderPath(id, 0, ptr::null_mut(),
                                              &mut result);
    }
    if error != winapi::S_OK {
        //let _ = io::stderr().write(format!("error: {}\n", error));
        return Err(());
    }
    unsafe {
        let mut len = 0;
        let mut cur = result;
        while *cur != 0 {
            len += 1;
            cur = cur.offset(1);
        }
        let os_string: OsString =
            OsStringExt::from_wide(slice::from_raw_parts(result, len));
        ole32::CoTaskMemFree(result as *mut _);
        Ok(os_string)
    }
}

#[cfg(target_os = "windows")]
pub fn user_data_dir(app: Option<&str>, author: Option<&str>, roaming: bool) -> PathBuf {
    let dir_id = if roaming { APPDATA_GUID } else { LOCAL_APPDATA_GUID };
    let mut data_dir = PathBuf::new();
    data_dir.push(get_dir(&dir_id).unwrap());
    if author.is_some() {
        data_dir.push(author.unwrap());
    }
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    data_dir
}

#[cfg(target_os = "windows")]
pub fn site_data_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    let mut data_dir = PathBuf::new();
    data_dir.push(get_dir(&COMMON_APPDATA_GUID).unwrap());
    if author.is_some() {
        data_dir.push(author.unwrap());
    }
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    data_dir
}

#[cfg(target_os = "windows")]
pub fn user_config_dir(app: Option<&str>, author: Option<&str>, roaming: bool) -> PathBuf {
    user_data_dir(app, author, roaming)
}

#[cfg(target_os = "windows")]
pub fn site_config_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    site_data_dir(app, author)
}

#[cfg(target_os = "windows")]
pub fn user_cache_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    let mut cache_dir = user_data_dir(app, author, false);
    cache_dir.push("Cache");
    cache_dir
}

#[cfg(target_os = "windows")]
pub fn user_log_dir(app: Option<&str>, author: Option<&str>) -> PathBuf {
    let mut log_dir = user_data_dir(app, author, false);
    log_dir.push("Logs");
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
                  super::user_data_dir(Some("AppDirs"), Some("djc"), false));
        to_stderr("site data dir",
                  super::site_data_dir(Some("AppDirs"), Some("djc")));
        to_stderr("user config dir",
                  super::user_config_dir(Some("AppDirs"), Some("djc"), false));
        to_stderr("site config dir",
                  super::site_config_dir(Some("AppDirs"), Some("djc")));
        to_stderr("user cache dir",
                  super::user_cache_dir(Some("AppDirs"), Some("djc")));
        to_stderr("user log dir",
                  super::user_log_dir(Some("AppDirs"), Some("djc")));
    }

}
