#[macro_use]
extern crate clap;
extern crate rayon;
#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate semver;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cli;
mod gitea;
mod github;

fn main() {
    let matches = cli::parse_cli();
    let host = matches.value_of("HOST").unwrap();
    let api_token = matches.value_of("GH_TOKEN");
    match rayon::join(
        || github::get_latest_release(api_token),
        || gitea::get_gitea_version(host),
    ) {
        (Ok(latest), Ok(current)) => if latest > current {
            println!(
                "A new version of gitea is available. Current: {}, latest: {}",
                current, latest
            );
        } else {
            println!("Gitea is up to date");
        },
        (Err(e), Ok(_)) => eprintln!("Cannot get latest gitea version: {:?}", e),
        (Ok(_), Err(e)) => eprintln!("Cannot get current gitea version: {:?}", e),
        (Err(e1), Err(e2)) => {
            eprintln!("Cannot get latest gitea version: {:?}", e1);
            eprintln!("Cannot get current gitea version: {:?}", e2);
        }
    }
    // eprintln!("{:?}", gitea::get_gitea_version("https://git.vapspace.org"));
}
