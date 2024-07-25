use std::fs;

use anyhow::Result;

use crate::config::Config;

use crate::utils::dirs;

pub fn init_config() -> Result<()> {
    let a= dirs::app_home_dir().or(None);
    println!("app_home_dir: {}", app_home_dir);

    Ok(())
}

pub fn init_resources() -> Result<()> {
    // "/Users/carlos/Library/Application Support/io.github.clash-verge-next.clash-verge-next"
    let app_dir = dirs::app_home_dir()?;
    // /Users/carlos/Codespaces/clash-verge-next/src-tauri/target/debug/resources
    let res_dir = dirs::app_resources_dir()?;

    if !app_dir.exists() {
        let _ = fs::create_dir_all(&app_dir);
    }
    if !res_dir.exists() {
        let _ = fs::create_dir_all(&res_dir);
    }

    let file_list = ["Country.mmdb", "geoip.dat", "geosite.dat"];

    for file in file_list.iter() {
        let src_path = res_dir.join(file);
        let dest_path = app_dir.join(file);

        let handle_copy = || {
            match fs::copy(&src_path, &dest_path) {
                Ok(_) => log::debug!(target: "app", "资源拷贝完成 '{file}'"),
                Err(err) => log::error!(target: "app", "资源拷贝失败 '{file}', {err}"),
            };
        };

        if src_path.exists() && !dest_path.exists() {
            handle_copy();
            continue;
        }

        let src_modified = fs::metadata(&src_path).and_then(|m| m.modified());
        let dest_modified = fs::metadata(&dest_path).and_then(|m| m.modified());

        match (src_modified, dest_modified) {
            (Ok(src_modified), Ok(dest_modified)) => {
                if src_modified > dest_modified {
                    handle_copy();
                } else {
                    log::debug!(target: "app", "skipping resource copy '{file}'");
                }
            }
            _ => {
                log::debug!(target: "app", "failed to get modified '{file}'");
                handle_copy();
            }
        };
    }

    Ok(())
}

pub fn startup_script() -> Result<()> {
    let path: Option<String> = {
        let verge = Config::verge();
        let verge = verge.latest();
        verge.startup_script.clone()
    };

    Ok(())
}
