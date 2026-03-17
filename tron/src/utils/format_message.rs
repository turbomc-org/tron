use crate::State;
use crate::models::{achievements::Achievements, player::Player, prefix::Prefix, team::Team};
use anyhow::Result;
use std::sync::Arc;
use tracing::error;

pub async fn format_message(
    state: &Arc<State>,
    player: &Player,
    message: &String,
) -> Result<String> {
    let mut prefix: Option<Prefix> = None;
    let mut team: Option<Team> = None;

    if let Some(prefix_id) = player.selected_prefix {
        match state.get_prefix(&prefix_id).await {
            Ok(p) => prefix = Some(p),
            Err(err) => error!("❌ Failed to get prefix for '{}': {}", player.username, err),
        };
    }

    if let Some(team_id) = player.team {
        match state.get_team(team_id) {
            Some(t) => team = Some(t),
            None => (),
        }
    }

    let overall_rank = state
        .leaderboards
        .overall
        .get_rank(&player.id)
        .await
        .unwrap_or(0) as u64;
    let kd_rank = state
        .leaderboards
        .kd
        .get_rank(&player.id)
        .await
        .unwrap_or(0) as u64;
    let kills_rank = state
        .leaderboards
        .kills
        .get_rank(&player.id)
        .await
        .unwrap_or(0) as u64;
    let deaths_rank = state
        .leaderboards
        .deaths
        .get_rank(&player.id)
        .await
        .unwrap_or(0) as u64;
    let coins_rank = state
        .leaderboards
        .coins
        .get_rank(&player.id)
        .await
        .unwrap_or(0) as u64;

    let mut emoji_part = String::new();

    emoji_part.push_str(&rank_emoji("\u{1F3C5}", overall_rank, "Overall Rank"));
    emoji_part.push_str(&rank_emoji("\u{1FA78}", kills_rank, "Most Kills"));
    emoji_part.push_str(&rank_emoji("\u{1F5E1}", kd_rank, "Best K/D Ratio"));
    emoji_part.push_str(&rank_emoji("\u{1F4A9}", deaths_rank, "Most Deaths"));
    emoji_part.push_str(&rank_emoji("\u{1FA99}", coins_rank, "Richest Player"));

    let achievement_display = get_best_achievement_display(&player.achievements);

    let prefix_or_rank = if let Some(p) = prefix.clone() {
        format!(
            "<hover:show_text:'<gray>Prefix:</gray> <{}>{}</{}>'><color:{}><i>{}</i></color></hover>",
            p.color, p.text, p.color, p.color, p.text
        )
    } else {
        let rank_str = format!("{:?}", player.rank);
        format!(
            "<hover:show_text:'<gray>Rank:</gray> <white>{}</white>'><bold>{}</bold></hover>",
            rank_str, rank_str
        )
    };

    let username_display = if let Some(t) = team.clone() {
        format!(
            "<hover:show_text:'<gray>Team:</gray> <{}>{}</{}>'><color:{}><bold>{}</bold></color></hover>",
            t.color, t.name, t.color, t.color, player.username
        )
    } else {
        format!("<bold>{}</bold>", player.username)
    };

    let final_message = format!(
        "{} {} {} <color:#750085><st>=</st></color> {} {}",
        emoji_part.trim(),
        achievement_display,
        prefix_or_rank,
        username_display,
        message
    );

    Ok(final_message)
}

fn rank_emoji(emoji: &str, rank: u64, hover_label: &str) -> String {
    if rank == 0 {
        return String::new();
    }

    let (color, place) = match rank {
        1 => ("#FFD700", "1st"),
        2 => ("#C0C0C0", "2nd"),
        3 => ("#b87333", "3rd"),
        _ => return String::new(), // only top 3
    };

    format!(
        "<hover:show_text:'<gray>{}</gray>: <{}>{}</{}>'><color:{}>{}</color></hover> ",
        hover_label, color, place, color, color, emoji
    )
}

fn get_best_achievement_display(achievements: &std::collections::HashSet<Achievements>) -> String {
    use Achievements::*;

    let mut best_value = 0;
    let mut best_type = None;

    for a in achievements {
        let (val, typ) = match a {
            MinerI => (1, "Miner"),
            MinerII => (2, "Miner"),
            MinerIII => (3, "Miner"),
            MinerIV => (4, "Miner"),
            MinerV => (5, "Miner"),
            MinerVI => (6, "Miner"),
            MinerVII => (7, "Miner"),
            MinerVIII => (8, "Miner"),
            MinerIX => (9, "Miner"),
            MinerX => (10, "Miner"),
            MinerXI => (11, "Miner"),
            MinerXII => (12, "Miner"),
            MinerXIII => (13, "Miner"),
            MinerXIV => (14, "Miner"),
            MinerXV => (15, "Miner"),
            MinerXVI => (16, "Miner"),
            MinerXVII => (17, "Miner"),
            MinerXVIII => (18, "Miner"),
            MinerXIX => (19, "Miner"),
            MinerXX => (20, "Miner"),

            WarriorI => (1, "Warrior"),
            WarriorII => (2, "Warrior"),
            WarriorIII => (3, "Warrior"),
            WarriorIV => (4, "Warrior"),
            WarriorV => (5, "Warrior"),
            WarriorVI => (6, "Warrior"),
            WarriorVII => (7, "Warrior"),
            WarriorVIII => (8, "Warrior"),
            WarriorIX => (9, "Warrior"),
            WarriorX => (10, "Warrior"),
            WarriorXI => (11, "Warrior"),
            WarriorXII => (12, "Warrior"),
            WarriorXIII => (13, "Warrior"),
            WarriorXIV => (14, "Warrior"),
            WarriorXV => (15, "Warrior"),
            WarriorXVI => (16, "Warrior"),
            WarriorXVII => (17, "Warrior"),
            WarriorXVIII => (18, "Warrior"),
            WarriorXIX => (19, "Warrior"),
            WarriorXX => (20, "Warrior"),

            BuilderI => (1, "Builder"),
            BuilderII => (2, "Builder"),
            BuilderIII => (3, "Builder"),
            BuilderIV => (4, "Builder"),
            BuilderV => (5, "Builder"),
            BuilderVI => (6, "Builder"),
            BuilderVII => (7, "Builder"),
            BuilderVIII => (8, "Builder"),
            BuilderIX => (9, "Builder"),
            BuilderX => (10, "Builder"),
            BuilderXI => (11, "Builder"),
            BuilderXII => (12, "Builder"),
            BuilderXIII => (13, "Builder"),
            BuilderXIV => (14, "Builder"),
            BuilderXV => (15, "Builder"),
            BuilderXVI => (16, "Builder"),
            BuilderXVII => (17, "Builder"),
            BuilderXVIII => (18, "Builder"),
            BuilderXIX => (19, "Builder"),
            BuilderXX => (20, "Builder"),
        };

        if val > best_value {
            best_value = val;
            best_type = Some(typ);
        }
    }

    if let Some(typ) = best_type {
        let roman = to_roman(best_value);
        let (symbol, color) = match typ {
            "Miner" => ("⛏", "green"),
            "Warrior" => ("⚔", "#fa5300"),
            "Builder" => ("⚒", "blue"),
            _ => ("❔", "gray"),
        };

        format!(
            "<hover:show_text:'{} {}'><color:{}>{}{}</color></hover>",
            typ, roman, color, symbol, roman
        )
    } else {
        "".to_string()
    }
}

fn to_roman(mut num: usize) -> String {
    let romans = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut result = String::new();
    for &(value, symbol) in romans.iter() {
        while num >= value {
            result.push_str(symbol);
            num -= value;
        }
    }
    result
}
