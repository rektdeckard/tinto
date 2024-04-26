use crate::app::Args;
use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use std::io::Write;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;
use toml::{self, Table};

const DEFAULT_CONFIG_FILE: &'static str = "tinto.toml";

#[derive(Debug, Clone)]
pub struct Config {
    file_path: PathBuf,
    dir_path: PathBuf,
    pub bridge_ip: IpAddr,
    pub app_key: String,
}

impl Config {
    pub fn try_init(args: &Args) -> Result<Self> {
        let dir_path = Config::ensure_dir()?;
        let file_path = dir_path.join(DEFAULT_CONFIG_FILE).to_owned();
        Config::init_logging(&dir_path);

        if args.addr.is_some() && args.key.is_some() {
            return Ok(Config {
                file_path,
                dir_path,
                bridge_ip: args.addr.unwrap(),
                app_key: args.key.clone().unwrap(),
            });
        }

        if file_path.exists() {
            let table = Self::read_config_toml(&file_path).expect("malformed config file");
            let bridge_ip = args.addr.clone().unwrap_or(
                IpAddr::from_str(
                    table["device"]["bridge_addr"]
                        .as_str()
                        .expect("no entry for bridge_addr"),
                )
                .expect("malformed bridge_addr"),
            );
            let app_key = args.key.clone().unwrap_or(
                table["device"]["app_key"]
                    .as_str()
                    .expect("no entry for app_key")
                    .to_owned(),
            );

            Ok(Config {
                file_path,
                dir_path,
                bridge_ip,
                app_key: app_key.to_string(),
            })
        } else {
            Err(anyhow::anyhow!("missing configuration"))
        }
    }

    fn init_logging(dir_path: &PathBuf) {
        if cfg!(debug_assertions) {
            let file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(dir_path.join("tinto.log"))
                .expect("could not open log file for witing");
            simplelog::WriteLogger::init(
                simplelog::LevelFilter::Info,
                simplelog::Config::default(),
                file,
            )
            .expect("could not initialize logger");
        }
    }

    fn ensure_dir() -> Result<PathBuf> {
        let dir_path = ProjectDirs::from("com", "rektsoft", "tinto")
            .expect("could not construct config dir")
            .config_local_dir()
            .to_owned();
        fs::create_dir_all(&dir_path)?;
        Ok(dir_path)
    }

    fn read_config_toml(file_path: &PathBuf) -> Result<Table> {
        let toml = fs::read_to_string(file_path)?;
        let table = toml.parse::<Table>()?;
        Ok(table)
    }

    pub fn write_config_toml(addr: &IpAddr, key: &str) -> Result<()> {
        let dir_path = Config::ensure_dir()?;
        let file_path = dir_path.join(DEFAULT_CONFIG_FILE).to_owned();
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("could not open config file for witing");

        let addr = addr.to_string();
        let key = key.to_string();
        let table = toml::toml! {
            [device]
            bridge_arr = addr
            app_key = key
        };
        file.write(table.to_string().as_bytes())?;
        Ok(())
    }
}
