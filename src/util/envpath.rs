
use std::path::Path;
use std::env;

const ATA_PATH_NAME: &'static str = "ATADB_PATH";
const APP_INFO: AppInfo = AppInfo { name: "atadb", author: "atadb" };

pub fn locate_on_db_path(dbname: String) -> Option<Path> {
    let mut paths = Vec::new();
    match env::var_os(ATA_PATH_NAME) {
        Some(value) => {
            for path in env::split_paths(&value) {
                if (exists(path)) {
                    paths.append(path);
                } else {
                    warn!("Directory {} was found in {}, but it cannot be found.", path.display(), ATA_PATH_NAME);
                }
            }
        }
        None => ()
    };
    match env::home_dir() {
        Some(path) => paths.append(path.join("atadbs")),
        None => (),
    }
    // APP_INFO todo
    match data_root(AppDataType::UserData) {
        Ok(path) => paths.append(path),
        Err(_) => (),
    }
    for path in paths {
        ()
    }
    Option::None
}
