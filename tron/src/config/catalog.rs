use crate::message;
use once_cell::sync::Lazy;
use text_placeholder::Template;

pub static RELEASE_NOTES: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ RELEASE NOTES, {{version}}",
        body: "Connection to the <gradient:#B200FF:#6A00A3>H01 Network</gradient> re-established.\n\
    <dark_gray>»</dark_gray> <gradient:#D66BFF:#8A2BE2>Season 6</gradient> systems online — enjoy your session, <light_purple><bold>player.</bold></light_purple>",
        actions: {
            discord: { kind: "url", value: "https://discord.gg/yourinvite", label: "Report bugs or updates on Discord" }
        }
    }
});
