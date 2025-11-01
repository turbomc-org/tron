pub fn calculate_overall(kills: u64, deaths: u64, coins: u64) -> f64 {
    let safe_deaths = deaths.max(1);
    let kd = kills as f64 / safe_deaths as f64;

    let score = (kills as f64 * 1.0) + (kd * 50.0) + (coins as f64 * 0.5);

    (score * 100.0).round() / 100.0
}

pub fn calculate_kd(kills: u64, deaths: u64) -> f64 {
    let safe_deaths = deaths.max(1);
    let kd = kills as f64 / safe_deaths as f64;

    (kd * 100.0).round() / 100.0
}
