use std::net::IpAddr;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "roi-parser")]
pub struct Options {
    #[structopt(short, long, default_value = "127.0.0.1")]
    address: IpAddr,

    #[structopt(short, long, default_value = "8080")]
    port: u16,

    #[structopt(short, long, parse(from_os_str), default_value = "db.sqlite")]
    database: PathBuf,

    #[structopt(short, long, default_value = "30")]
    read_timeout: u64,

    #[structopt(short, long, default_value = "10")]
    query_interval: u64,

    #[structopt(short, long, default_value = "1800")]
    update_interval: u64,

    #[structopt(short, long)]
    no_worker: bool,
}

impl Options {
    pub fn address(&self) -> &IpAddr {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database(&self) -> &Path {
        &self.database
    }

    pub fn read_timeout(&self) -> u64 {
        self.read_timeout
    }

    pub fn query_interval(&self) -> u64 {
        self.query_interval
    }

    pub fn update_interval(&self) -> u64 {
        self.update_interval
    }

    pub fn no_worker(&self) -> bool {
        self.no_worker
    }
}
