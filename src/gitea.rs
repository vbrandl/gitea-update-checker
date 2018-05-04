use failure::Error;
use semver::Version;

const GITEA_VERSION_API: &str = "/api/v1/version";

#[derive(Deserialize)]
struct GiteaVersion {
    version: String,
}

pub fn get_gitea_version(host: &str) -> Result<Version, Error> {
    let resp: GiteaVersion = ::reqwest::get(&format!("{}{}", host, GITEA_VERSION_API))?.json()?;
    Ok(Version::parse(&resp.version)?)
}
