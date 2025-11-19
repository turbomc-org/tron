use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReleaseConfig {
    pub note: String,
}

pub static RELEASE_CONFIG: Lazy<ReleaseConfig> = Lazy::new(|| ReleaseConfig {
    note: r#"
        [0.1.5] H01 VERSION
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
        1. Scoreboard button not working on toggle
        2. Team invite command

        pending:-
        1. Password authentication system
        2. Shop
        3. Achievements notification
        "#
    .to_string(),
});
