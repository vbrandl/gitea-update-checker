pub fn parse_cli() -> ::clap::ArgMatches<'static> {
    clap_app!(gitea_update_checker =>
              (version: crate_version!())
              (author: crate_authors!())
              (about: "Checks if a Gitea instance is up to date")
              (@arg HOST: -h --host +takes_value +required "The Gitea host")
              (@arg GH_TOKEN: -t --token +takes_value "Github API token to circumvent API limits")
             ).get_matches()
}
