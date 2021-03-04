use actix_web_static_files::NpmBuild;

fn main() {
	NpmBuild::new("./web/")
		.install()
		.unwrap()
		.run("build")
		.unwrap()
		.target("./web/dist")
		.to_resource_dir()
		.build()
		.unwrap();
}
