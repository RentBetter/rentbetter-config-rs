extern crate envy;

extern crate toml;
extern crate dotenv;

use std::io::Read;

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
