use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReleaseConfig {
    pub note: String,
}

pub static PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static RELEASE_CONFIG: Lazy<ReleaseConfig> = Lazy::new(|| {
    let note = format!(
        r#"
[{version}] H01 VERSION
new features:-
1. Achievement title notification
2. New team related commands
3. New prefixes
4. Redeem codes

fixes:-
1. Tab list leaderboard
2. Scoreboard
3. Team commands

pending:-
1. Shop
2. Pvp server
"#,
        version = PACKAGE_VERSION
    );

    ReleaseConfig { note }
});
