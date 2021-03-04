use std::{
	net::{IpAddr, Ipv4Addr},
	path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "screeen", about = "All-in-one self hosted media system")]
pub struct Cli {
	/// Server IP address
	#[structopt(short, long, default_value = "0.0.0.0", env = "SCREEEN_ADDRESS")]
	pub address: IpAddr,

	/// Path to the config file
	#[structopt(
		short = "c",
		long = "config",
		default_value = "./config.toml",
		env = "SCREEEN_CONFIG"
	)]
	pub config_path: PathBuf,

	/// Media directory to serve
	#[structopt(short, long, default_value = "./media/", env = "SCREEEN_MEDIA")]
	pub media: PathBuf,

	/// Server port
	#[structopt(short, long, default_value = "5445", env = "SCREEEN_PORT")]
	pub port: u16,

	/// Verbose logging
	#[structopt(short, long)]
	pub verbose: bool,
}

impl Default for Cli {
	fn default() -> Self {
		Cli {
			address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
			config_path: PathBuf::from("./screeen.toml"),
			media: PathBuf::from("./media"),
			port: 5445,
			verbose: false,
		}
	}
}
