use std::{env, path::Path};

use crate::addon::construct_addon;

mod addon;

fn main() {

    let wow_path = env::var("CLASSIC_WOW_PATH").expect("Env Variable: \"CLASSIC_WOW_PATH\" not set. Set this environment variable to continue.");
    let addon_path_str = wow_path + "/Interface/AddOns/";
    let addons = Path::new(&addon_path_str);

    for entry in addons.read_dir().expect("Can't find entry") {
        if let Ok(entry) = entry {
            let addon = construct_addon(entry.path().as_path());

            if addon.version.is_some() {
                println!("{}: v{}", addon.title, addon.version.unwrap())
            }
        }
    }
}