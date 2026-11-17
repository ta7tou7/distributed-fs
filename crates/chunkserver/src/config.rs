use std::io::ErrorKind;

pub struct Config {
    pub port: u16,
    pub master_host: String,
    pub master_port: u16,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, std::io::Error> {
        args.next();
        let port = match args.next() {
            Some(arg) => match arg.parse::<u16>() {
                Ok(port) => port,
                Err(_) => return Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid port")),
            }
            None => return Err(std::io::Error::new(ErrorKind::InvalidInput, "Missing port")),
        };
        let master_host = match args.next() {
            Some(arg) => arg,
            None => return Err(std::io::Error::new(ErrorKind::InvalidInput, "Missing master host")),
        };
        let master_port = match args.next() {
            Some(arg) => match arg.parse::<u16>() {
                Ok(port) => port,
                Err(_) => return Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid master port")),
            },
            None => return Err(std::io::Error::new(ErrorKind::InvalidInput, "Missing master port")),
        };
        Ok(Config{port, master_host, master_port})
    }
}