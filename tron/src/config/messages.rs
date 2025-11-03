use once_cell::sync::Lazy;
use text_placeholder::Template;

pub static WELCOME_BACK: Lazy<Template<'static>> = Lazy::new(|| {
    Template::new(
        "<gradient:#C724B1:#7A00FF><bold>⚡ WELCOME BACK, {{username}}</bold></gradient>\n\
         <gray>Connection to the <gradient:#B200FF:#6A00A3>H01 Network</gradient> re-established.</gray>\n\
         <dark_gray>»</dark_gray> <gradient:#D66BFF:#8A2BE2>Season 6</gradient> systems online — enjoy your session, <light_purple><bold>player.</bold></light_purple>\n\
         <dark_gray>»</dark_gray> <click:open_url:'https://discord.gg/yourinvite'><u><gradient:#C724B1:#7A00FF>Report bugs or updates on Discord</gradient></u></click>",
    )
});

pub static WELCOME_FIRST_TIME: Lazy<Template<'static>> = Lazy::new(|| {
    Template::new(
        "<gradient:#C724B1:#7A00FF><bold>⛓ WELCOME, {{username}}</bold></gradient>\n\
        <gray>You've entered the <bold><gradient:#B200FF:#6A00A3>H01 Network</gradient></bold> for the first time.</gray>\n\
        <dark_gray>»</dark_gray> <gradient:#D66BFF:#8A2BE2>Season 6</gradient> has begun — your legacy starts <light_purple><bold>now.</bold></light_purple>\n\
        <dark_gray>»</dark_gray> <click:open_url:'https://discord.gg/yourinvite'><u><gradient:#C724B1:#7A00FF>Join the Grid (Discord)</gradient></u></click>",
    )
});

pub static BALANCE: Lazy<Template<'static>> = Lazy::new(|| {
    Template::new(
        "<gradient:#C724B1:#7A00FF><bold>¤ CREDIT BALANCE</bold></gradient>\n\
         <gray>You currently have <white><bold>{{balance}}</bold></white> Hash-Coins.</gray>\n\
         <dark_gray>»</dark_gray> <click:run_command:'/shop'><u><gradient:#B200FF:#6A00A3>Access the Network Market</gradient></u></click>",
    )
});

pub static TRANSFERRED: Lazy<Template<'static>> = Lazy::new(|| {
    Template::new(
        "<gradient:#C724B1:#7A00FF><bold>✅ CREDITS TRANSFERRED</bold></gradient>\n\
         <gray>You have successfully transferred <white><bold>{{amount}}</bold></white> Hash-Coins to <white><bold>{{target}}</bold></white>.</gray>\n\
         <dark_gray>»</dark_gray> <click:run_command:'/balance'><u><gradient:#B200FF:#6A00A3>Check your new balance</gradient></u></click>",
    )
});

pub static INCOMING_TRANSFER: Lazy<Template<'static>> = Lazy::new(|| {
    Template::new(
        "<gradient:#C724B1:#7A00FF><bold>✉️ INCOMING TRANSFER</bold></gradient>\n\
         <gray>You have received <white><bold>{{amount}}</bold></white> Hash-Coins from <white><bold>{{sender}}</bold></white>.</gray>\n\
         <dark_gray>»</dark_gray> <click:run_command:'/balance'><u><gradient:#B200FF:#6A00A3>Check your new balance</gradient></u></click>",
    )
});
