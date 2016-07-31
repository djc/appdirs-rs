extern crate shell32;
extern crate winapi;
extern crate ole32;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::slice;


static APPDATA_GUID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 1052149211,
    Data2: 26105,
    Data3: 19702,
    Data4: [160, 58, 227, 239, 101, 114, 159, 61],
};

static COMMON_APPDATA_GUID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 1655397762,
    Data2: 64961,
    Data3: 19907,
    Data4: [169, 221, 7, 13, 29, 73, 93, 151],
};

static LOCAL_APPDATA_GUID: winapi::shtypes::KNOWNFOLDERID = winapi::shtypes::KNOWNFOLDERID {
    Data1: 4055050117,
    Data2: 28602,
    Data3: 20431,
    Data4: [157, 85, 123, 142, 127, 21, 112, 145],
};

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

pub fn user_data_dir(app: Option<&str>, author: Option<&str>, roaming: bool)
                     -> Result<PathBuf, ()> {
    let dir_id = if roaming { APPDATA_GUID } else { LOCAL_APPDATA_GUID };
    let mut data_dir = PathBuf::new();
    data_dir.push(try!(get_dir(&dir_id)));
    if author.is_some() {
        data_dir.push(author.unwrap());
    }
    if app.is_some() {
        data_dir.push(app.unwrap());
    }
    Ok(data_dir)
}

pub fn site_data_dir(app: Option<&str>, author: Option<&str>) -> Result<PathBuf, ()> {
    let mut data_dir = PathBuf::new();
    data_dir.push(try!(get_dir(&COMMON_APPDATA_GUID)));
    if author.is_some() {
        data_dir.push(author.unwrap());
    }
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

pub fn user_cache_dir(app: Option<&str>, author: Option<&str>) -> Result<PathBuf, ()> {
    let cache_dir = user_data_dir(app, author, false);
    match cache_dir {
        Ok(mut cache_dir) => { cache_dir.push("Cache"); Ok(cache_dir) },
        Err(err) => Err(err),
    }
}

pub fn user_log_dir(app: Option<&str>, author: Option<&str>) -> Result<PathBuf, ()> {
    let log_dir = user_data_dir(app, author, false);
    match log_dir {
        Ok(mut log_dir) => { log_dir.push("Logs"); Ok(log_dir) },
        Err(err) => Err(err),
    }
}
