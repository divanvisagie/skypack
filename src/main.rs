mod mod_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Skypack")
        .version("1.0")
        .author("Divan Visagie")
        .about("Downloads and manages Skyrim mods from Nexus Mods")
        .subcommand(clap::SubCommand::with_name("download")
            .about("Downloads a mod")
            .arg(clap::Arg::with_name("mod_id")
                 .required(true)
                 .takes_value(true)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("download") {
        mod_manager::download_mod(matches.value_of("mod_id").unwrap()).await?;
    }

    Ok(())
}
