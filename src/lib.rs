extern crate envy;

extern crate toml;
extern crate dotenv;
extern crate num_cpus;

use std::io::Read;
use std::env;

use dotenv::dotenv;

pub fn acquire_config<T>(config_fname: String) -> T
                         where for<'de> T: toml::macros::Deserialize<'de> {
    match std::fs::File::open(&*config_fname) {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            return toml::from_slice(&buffer).unwrap()
        },
        Err(_) => {
            dotenv().ok();

            match envy::from_env::<T>() {
                Ok(config) => return config,
                Err(error) => panic!("{:#?}", error)
            }
        },
    };
}

pub fn default_worker() -> usize {
    num_cpus::get() * 8
}

pub fn default_db_url() -> String {
    static DB_URL: &str = "DATABASE_URL";
    match env::var_os(DB_URL) {
        Some(db_url) => return db_url.to_string_lossy().to_string(),
        None => {
            match env::var_os("POSTGRES_USER") {
                Some(user) => {
                    let user_s = user.to_string_lossy();
                    return format!("postgres://{}:{}@{}/{}", user_s, env::var("POSTGRES_PASSWORD").unwrap(), "localhost", env::var("POSTGRES_DB").unwrap_or((&*user_s).into()).to_string())
                },
                None => panic!(format!("{} must be defined", DB_URL))
            }
        }
    }
}
