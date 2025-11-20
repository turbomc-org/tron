use once_cell::sync::Lazy;
use text_placeholder::Template;

pub static ACHIEVEMENT_TITLE: Lazy<Template<'static>> =
    Lazy::new(|| Template::new("\u{2B06} {{achievement}}"));
pub static ACHIEVEMENT_SUBTITLE: Lazy<Template<'static>> =
    Lazy::new(|| Template::new("{{coins}} Coins added to your account"));
