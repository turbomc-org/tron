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
features:-
1. Direct message to friend using /dm
2. Team chat using /tc
3. Toggle scoreboard using /scoreboard
4. Buy prefix using prefix command
5. Report for bug using /bug <description>
6. Report players using /report <player_name> <reason>
7. Use leaderboard command to view leaderboard and your rank
8. Press tab to view leaderboard directly

fixes:-
1. Redeem codes
2. Team invite command

pending:-
1. Password authentication system
2. Shop
3. Achievements notification
"#,
        version = PACKAGE_VERSION
    );

    ReleaseConfig { note }
});
