use crate::cli::Cli;
use actix_web::{App, HttpServer};
use log::info;
use std::{io::Result as IOResult, net::IpAddr};

#[cfg(feature = "web")]
use actix_web_static_files;
#[cfg(feature = "web")]
use screeen_web::generate;

pub struct Server {
	address: IpAddr,
	port: u16,
}

impl Server {
	/// Create a new server instance
	pub fn new(cli: Cli) -> Server {
		info!("Starting server...");

		Server {
			address: cli.address,
			port: cli.port,
		}
	}

	/// Start the Actix HTTP web server
	pub async fn run(&self) -> IOResult<()> {
		HttpServer::new(move || {
			let mut app = App::new();

			#[cfg(feature = "web")]
			{
				let generated = generate();
				app = app.service(
					actix_web_static_files::ResourceFiles::new("/", generated)
						.resolve_not_found_to_root(),
				);
			}

			app
		})
		.bind(format!("{}:{}", self.address, self.port))?
		.run()
		.await
	}
}
