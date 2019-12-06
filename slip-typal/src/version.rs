use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    release: Option<String>,
    build: Option<String>,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let major_cmp = self.major.cmp(&other.major);
        let minor_cmp = self.minor.cmp(&other.minor);
        let patch_cmp = self.patch.cmp(&other.patch);
        let release_cmp = || self.release.cmp(&other.release);

        major_cmp
            .then(minor_cmp)
            .then(patch_cmp)
            .then_with(release_cmp)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Version {}

lazy_static! {
    static ref VERSION_MATCH: Regex = Regex::new(r#"^(?P<major>\d+).(?P<minor>\d+).(?P<patch>\d+)(?:-(?P<release>[\w.]+))?(?:\+(?P<build>[\w.]+))?$"#).unwrap();
}

#[derive(Debug, Fail, Copy, Clone, Serialize, Deserialize)]
pub enum VersionError {
    #[fail(display = "string didn't match version format")]
    MatchError,
    #[fail(display = "could not convert component to integer")]
    IntError,
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = &*VERSION_MATCH as &Regex;
        if let Some(caps) = m.captures(s) {
            let major = caps["major"]
                .parse::<u32>()
                .map_err(|_| VersionError::IntError)?;
            let minor = caps["minor"]
                .parse::<u32>()
                .map_err(|_| VersionError::IntError)?;
            let patch = caps["patch"]
                .parse::<u32>()
                .map_err(|_| VersionError::IntError)?;
            let release = caps.name("release").map(|m| m.as_str().to_string());
            let build = caps.name("build").map(|m| m.as_str().to_string());
            Ok(Version {
                major,
                minor,
                patch,
                release,
                build,
            })
        } else {
            Err(VersionError::MatchError)
        }
    }
}
