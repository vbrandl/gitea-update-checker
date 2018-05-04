use failure::Error;
use semver::Version;

const GITHUB_API: &str = "https://api.github.com/repos/go-gitea/gitea/releases";

#[derive(Deserialize, Debug)]
struct GithubRelease {
    tag_name: String,
}

#[derive(Debug, Fail)]
pub enum GithubApiError {
    #[fail(display = "Cannot parse API response")]
    ParseApiResponse,
}

pub fn get_latest_release(api_token: Option<&str>) -> Result<Version, Error> {
    let resp: Vec<GithubRelease> = if let Some(api_token) = api_token {
        ::reqwest::get(&format!("{}?access_token={}", GITHUB_API, api_token))
    } else {
        ::reqwest::get(GITHUB_API)
    }?.json()?;
    Ok(resp.into_iter()
        .flat_map(|s| {
            let tag = if s.tag_name.starts_with('v') {
                &s.tag_name[1..]
            } else {
                &s.tag_name
            };
            Version::parse(tag)
        })
        .filter(|v| !v.is_prerelease())
        .nth(0)
        .ok_or(GithubApiError::ParseApiResponse)?)
}
