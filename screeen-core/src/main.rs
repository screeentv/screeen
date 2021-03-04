mod cli;
mod server;

use crate::{cli::Cli, server::Server};
use log::error;
use std::{env, io::Result};
use structopt::StructOpt;

#[actix_rt::main]
async fn main() -> Result<()> {
	let cli: Cli = Cli::from_args();

	if cli.verbose && env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "info");
		env_logger::init();
	}

	let result = Server::new(cli).run().await;

	match result {
		Ok(_) => std::process::exit(0),
		Err(e) => {
			error!("Server failed to start: {}", e);
			std::process::exit(-1);
		},
	}
}
