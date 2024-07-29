use std::fs;

use anyhow::Result;

use crate::config::{Config, IClashTemp, IProfiles, IVerge};

use crate::utils::{dirs, help};

pub fn init_config() -> Result<()> {
    // "/Users/carlos/Library/Application Support/io.github.clash-verge-next.clash-verge-next"
    crate::log_err!(dirs::app_home_dir().map(|app_dir| {
        if !app_dir.exists() {
            let _ = fs::create_dir_all(app_dir);
        }
    }));

    // "/Users/carlos/Library/Application Support/io.github.clash-verge-next.clash-verge-next/profiles"
    crate::log_err!(dirs::app_profiles_dir().map(|profiles_dir| {
        if !profiles_dir.exists() {
            let _ = fs::create_dir_all(profiles_dir);
        }
    }));

    // "/Users/carlos/Library/Application Support/io.github.clash-verge-next.clash-verge-next/config.yaml"
    crate::log_err!(dirs::clash_path().map(|path| {
        if !path.exists() {
            help::save_yaml(&path, &IClashTemp::template().0, Some("# Clash Vergeasu"))?;
        }

        <Result<()>>::Ok(())
    }));

    crate::log_err!(dirs::verge_path().map(|path| {
        if !path.exists() {
            help::save_yaml(&path, &IVerge::template(), Some("# Clash Verge"))?;
        }
        <Result<()>>::Ok(())
    }));

    crate::log_err!(dirs::profiles_path().map(|path| {
        if !path.exists() {
            help::save_yaml(&path, &IProfiles::template(), Some("# Clash Verge"))?;
        }
        <Result<()>>::Ok(())
    }));

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

    // 拷贝文件到 app_dir
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
    let path = {
        let verge = Config::verge();
        let verge = verge.latest();
        verge.startup_script.clone().unwrap_or("".to_string())
    };

    if !path.is_empty() {}

    Ok(())
}
