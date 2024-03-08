use clap::{App, Arg, SubCommand};

fn fn main() {
    let matches = App::new("Kolovrat CLI")
        .version("0.1.0")
        .author("Sanjin Gumbarevic <sanjin.gumbarevic@gmail.com>")
        .about("Controls the Kolovrat Distrubuted Web Crawling and Content Analysis Platform")
        .subcommand(SubCommand::with_name("crawl")
            .about("Initiates a web crawling operation")
            .arg(Arg::with_name("url")
                 .help("The url to start crawling from")
                 .required(true)
                 .index(1)))
        .subcommand(SubCommand::with_name("generate")
                    .about("Generates the static site with analysis results"))
        .get_matches();

    match matches.subcommand() {
        ("crawl", Some(crawl_matches)) => {
            let url = crawl_matches.value_of("url").unwrap();
            println!("Starting crowling at URL: {}", url);
            //TODO invoke web crowling functionality
        },
        ("generate", Some(_)) => {
            println!("Generating static website...");
            //TODO invoke website generator
        };
        _ => {}
    }
}
