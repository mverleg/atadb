use app_dirs::{app_dir, AppDataType, AppInfo};
use std::env;
use std::path::PathBuf;

const ATA_PATH_NAME: &'static str = "ATADB_PATH";
const APP_INFO: AppInfo = AppInfo {
    name: "atadb",
    author: "atadb",
};

pub fn locate_on_db_path(dbname: String) -> Option<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    match env::var_os(ATA_PATH_NAME) {
        Some(value) => {
            for path in env::split_paths(&value) {
                if !path.exists() {
                    paths.push(path);
                } else {
                    warn!(
                        "Directory \"{}\" was found in environment {}, but it cannot be found.",
                        path.display(),
                        ATA_PATH_NAME
                    );
                }
            }
        }
        None => (),
    };
    // TODO @mverleg: change this to something like ~/.local/share/atadb
    match env::home_dir() {
        Some(path) => paths.push(path.join(".atadb").join("dbs")),
        None => (),
    }
    // APP_INFO todo
    match app_dir(AppDataType::UserData, &APP_INFO, "dbs") {
        Ok(path) => paths.push(path),
        Err(err) => warn!("Could not create app data directory ({:?})", err),
    }
    let mut found: Option<PathBuf> = None;
    for path in paths {
        let dbpath = path.join(&dbname).with_extension(".atadb");
        if dbpath.exists() {
            if let Some(prev) = &found {
                warn!(
                    "Found database {} at \"{}\", but another one was found at \"{}\"",
                    dbname,
                    dbpath.display(),
                    prev.display()
                );
                continue;
            }
            found = Some(dbpath);
        } else {
            debug!("Searching for {} at \"{}\"", dbname, dbpath.display());
        }
    }
    found
}
