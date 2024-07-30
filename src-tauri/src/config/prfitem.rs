use anyhow::{bail, Context, Result};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;
use sysproxy::Sysproxy;

use crate::utils::help;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrfOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_proxy: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PrfItem {
    pub uid: Option<String>,

    /// profile file
    pub file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<PrfOption>,

    /// file data
    #[serde(skip)]
    pub file_data: Option<String>,
}

impl PrfItem {
    pub async fn from_url(
        url: &str,
        name: Option<String>,
        desc: Option<String>,
        option: Option<PrfOption>,
    ) -> Result<PrfItem> {
        let opt_ref = option.as_ref();
        let with_proxy = opt_ref.map_or(false, |o| o.with_proxy.unwrap_or(false));
        let mut builder = reqwest::ClientBuilder::new().use_rustls_tls().no_proxy();

        // 使用系统代理
        if with_proxy {
            if let Ok(p @ Sysproxy { enable: true, .. }) = Sysproxy::get_system_proxy() {
                let proxy_scheme = format!("http://{}:{}", p.host, p.port);

                if let Ok(proxy) = reqwest::Proxy::http(&proxy_scheme) {
                    builder = builder.proxy(proxy);
                }

                if let Ok(proxy) = reqwest::Proxy::https(&proxy_scheme) {
                    builder = builder.proxy(proxy);
                }

                if let Ok(proxy) = reqwest::Proxy::all(&proxy_scheme) {
                    builder = builder.proxy(proxy);
                }
            }
        }

        let resp = builder.build()?.get(url).send().await?;
        let status_code = resp.status();
        if !StatusCode::is_success(&status_code) {
            bail!("订阅链接获取失败 status code: {status_code}")
        }

        let header = resp.headers();

        let uid = help::get_uid("r");
        let file = format!("{uid}.yaml");
        let data = resp.text_with_charset("uft-8").await?;

        let data = data.trim_start_matches('\u{feff}');
        let yaml = serde_yaml::from_str::<Mapping>(data).context("订阅是无效的yaml")?;

        if !yaml.contains_key("proxies") {
            bail!("订阅没有`proxies`或`proxy-providers`")
        }

        Ok(PrfItem {
            uid: Some(uid),
            file: Some(file),
            url: Some(url.into()),
            option,
            file_data: Some(data.into()),
        })
    }
}
