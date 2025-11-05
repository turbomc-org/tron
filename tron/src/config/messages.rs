#![allow(unused)]

use crate::message;
use once_cell::sync::Lazy;
use text_placeholder::Template;

pub static WELCOME_BACK: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ WELCOME BACK, {{username}}",
        body: "Connection to the <gradient:#B200FF:#6A00A3>H01 Network</gradient> re-established.\n\
    <dark_gray>»</dark_gray> <gradient:#D66BFF:#8A2BE2>Season 6</gradient> systems online — enjoy your session, <light_purple><bold>player.</bold></light_purple>",
        actions: {
            discord: { kind: "url", value: "https://discord.gg/yourinvite", label: "Report bugs or updates on Discord" }
        }
    }
});

pub static WELCOME_FIRST_TIME: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⛓ WELCOME, {{username}}",
        body: r#"You've entered the <bold><gradient:#B200FF:#6A00A3>H01 Network</gradient></bold> for the first time.
    <dark_gray>»</dark_gray> <gradient:#D66BFF:#8A2BE2>Season 6</gradient> has begun — your legacy starts <light_purple><bold>now.</bold></light_purple>"#,
        actions: {
            discord: { kind: "url", value: "https://discord.gg/yourinvite", label: "Join the Grid (Discord)" }
        }
    }
});

pub static BALANCE: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "¤ CREDIT BALANCE",
        body: r#"You currently have <white><bold>{{balance}}</bold></white> Hash-Coins."#,
        actions: {
            open_shop: { kind: "command", value: "/shop", label: "Access the Network Market" }
        }
    }
});

pub static TRANSFERRED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ CREDITS TRANSFERRED",
        body: r#"You have successfully transferred <white><bold>{{amount}}</bold></white> Hash-Coins to <white><bold>{{target}}</bold></white>."#,
        actions: {
            check_balance: { kind: "command", value: "/balance", label: "Check your new balance" }
        }
    }
});

pub static INCOMING_TRANSFER: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✉️ INCOMING TRANSFER",
        body: r#"You have received <white><bold>{{amount}}</bold></white> Hash-Coins from <white><bold>{{sender}}</bold></white>."#,
        actions: {
            check_balance: { kind: "command", value: "/balance", label: "Check your new balance" }
        }
    }
});

pub static FRIEND_CONNECTED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ FRIEND CONNECTED",
        body: r#"You are now friends with <white><bold>{{sender}}</bold></white>."#,
        actions: {
            open_friends_list: { kind: "command", value: "/friends", label: "Open Friends List" }
        }
    }
});

pub static FRIEND_REQUEST_ACCEPTED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ FRIEND REQUEST ACCEPTED",
        body: r#"<white><bold>{{username}}</bold></white> has accepted your connection request."#,
        actions: {
            view_your_friends: { kind: "command", value: "/friend list", label: "View your friends" }
        }
    }
});

pub static NO_CONNECTIONS: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "ℹ NO CONNECTIONS",
        body: r#"Your friend network is empty. Establish new connections.
    <dark_gray>»</dark_gray> Use <white>/friend add <user></white> to send a request."#
    }
});

pub static FRIEND_NETWORK: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌐 FRIEND NETWORK",
        body: r#"Displaying <white>{{count}}</white> connected users:
    {{friend_list}}
    <dark_gray>»</dark_gray> Use <white>/friend remove <user></white> to disconnect."#
    }
});

pub static NO_INCOMING_FRIEND_REQUESTS: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "📭 NO INCOMING FRIEND REQUESTS",
        body: r#"Your <gradient:#B200FF:#6A00A3>H01 Network</gradient> inbox is currently empty."#,
        actions: {
            send_a_new_request: { kind: "command", value: "/friends", label: "Send a new request" }
        }
    }
});

pub static INCOMING_FRIEND_REQUESTS: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "📨 INCOMING FRIEND REQUESTS",
        body: r#"You have <light_purple><bold>{{count}}</bold></light_purple> pending connection{{s}}
    on the <gradient:#B200FF:#6A00A3>H01 Network</gradient>.
    {{list}}"#,
        actions: {
            accept_all: { kind: "command", value: "/friends accept_all" },
            deny_all: { kind: "command", value: "/friends deny_all" }
        }
    }
});

pub static FRIEND_REQUEST_REJECTED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ FRIEND REQUEST REJECTED",
        body: r#"You have rejected the friend request from <white><bold>{{sender}}</bold></white>."#,
        actions: {
            view_pending_requests: { kind: "command", value: "/friends", label: "View pending requests" }
        }
    }
});

pub static FRIEND_REQUEST_DECLINED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ FRIEND REQUEST DECLINED",
        body: r#"Your friend request to <white><bold>{{username}}</bold></white> was rejected."#
    }
});

pub static FRIEND_REMOVED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ FRIEND REMOVED",
        body: r#"You have successfully removed <white><bold>{{target}}</bold></white> from your friend list."#,
        actions: {
            view_remaining_friends: { kind: "command", value: "/friends", label: "View remaining friends" }
        }
    }
});

pub static NEW_FRIEND_REQUEST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ NEW FRIEND REQUEST ⚡",
        body: r#"<white><bold>{{sender}}</bold></white> wants to connect with you on the <gradient:#B200FF:#6A00A3>H01 Network</gradient>.
    <dark_gray>»</dark_gray> <click:run_command:'/friend accept {{sender}}'><u><gradient:#8A2BE2:#C724B1>[ ACCEPT ]</gradient></u></click>
    <click:run_command:'/friend deny {{sender}}'><u><gradient:#7A00FF:#4B0082>[ DENY ]</gradient></u></click>
    <dark_gray>»</dark_gray> Manage requests via <light_purple>/friends</light_purple>"#
    }
});

pub static FRIEND_REQUEST_SENT: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ FRIEND REQUEST SENT",
        body: r#"Your request has been transmitted to <white><bold>{{receiver}}</bold></white> via the <gradient:#B200FF:#6A00A3>H01 Network</gradient>.
    <dark_gray>»</dark_gray> <light_purple>Awaiting connection response...</light_purple>"#,
        actions: {
            view_pending_requests: { kind: "command", value: "/friends", label: "View pending requests" }
        }
    }
});

pub static ALREADY_OWN_PREFIX: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ DUPLICATE ASSET",
        body: r#"You have already unlocked this network identifier."#,
        actions: {
            view_your_collection: { kind: "command", value: "/prefixes", label: "View your collection" }
        }
    }
});

pub static INSUFFICIENT_CREDITS: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ INSUFFICIENT CREDITS",
        body: r#"Your balance is too low to acquire this asset. You need <white>{{credits}}</white> more credits."#,
        actions: {
            check_balance: { kind: "command", value: "/balance", label: "Check your balance" }
        }
    }
});

pub static ASSET_ACQUIRED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ ASSET ACQUIRED",
        body: r#"You purchased the <color:{{color}}>{{text}}</color> prefix for <white>{{price}}</white> credits."#,
        actions: {
            equip: { kind: "command", value: "/prefix set {{name}}", label: "Click to equip" }
        }
    }
});

pub static IDENTIFIER_REGISTERED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ IDENTIFIER REGISTERED",
        body: r#"Successfully registered the <color:{{color}}>{{text}}</color> identifier on the network.
    <dark_gray>»</dark_gray> It is now available for players to acquire."#
    }
});

pub static ASSET_PURGED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ ASSET PURGED",
        body: r#"Successfully purged the <white><bold>{{name}}</bold></white> identifier from the network."#,
        actions: {
            view_remaining_assets: { kind: "command", value: "/shop prefixes", label: "View remaining assets" }
        }
    }
});

pub static IDENTIFIER_EQUIPPED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ IDENTIFIER EQUIPPED",
        body: r#"You have equipped the <color:{{color}}>{{text}}</color> network identifier."#,
        actions: {
            unequip: { kind: "command", value: "/prefix unequip", label: "Click to unequip" }
        }
    }
});

pub static ACTIVE_IDENTIFIER: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "ℹ️ ACTIVE IDENTIFIER",
        body: r#"Your currently equipped network identifier is <color:{{color}}>{{text}}</color>."#,
        actions: {
            unequip: { kind: "command", value: "/prefix unequip", label: "Click to unequip" }
        }
    }
});

pub static MARKET_DATABASE_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "ℹ️ MARKET DATABASE EMPTY",
        body: r#"There are currently no network identifiers available for acquisition.
    <dark_gray>»</dark_gray> Please check back later."#
    }
});

pub static NETWORK_MARKET_IDENTIFIERS: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌐 NETWORK MARKET: IDENTIFIERS",
        body: r#"Displaying all available network assets:
    {{list}}"#,
        actions: {
            view_your_owned_assets: { kind: "command", value: "/prefixes", label: "View your owned assets" }
        }
    }
});

pub static NO_ASSETS_UNLOCKED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "ℹ️ NO ASSETS UNLOCKED",
        body: r#"Your collection is empty. Acquire identifiers from the network market."#,
        actions: {
            browse_the_network_market: { kind: "command", value: "/prefix list", label: "Browse the Network Market" }
        }
    }
});

pub static IDENTIFIER_COLLECTION: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🎨 IDENTIFIER COLLECTION",
        body: r#"Displaying your <white>{{count}}</white> unlocked network assets:
    {{list}}"#,
        actions: {
            acquire_more_assets: { kind: "command", value: "/prefix list", label: "Acquire more assets" }
        }
    }
});

pub static NO_ACTIVE_IDENTIFIER: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "ℹ️ NO ACTIVE IDENTIFIER",
        body: r#"You do not have a network identifier equipped to unequip."#,
        actions: {
            select_an_identifier_to_equip: { kind: "command", value: "/prefixes", label: "Select an identifier to equip" }
        }
    }
});

pub static IDENTIFIER_UNEQUIPPED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ IDENTIFIER UNEQUIPPED",
        body: r#"Your network identifier has been reset to default."#,
        actions: {
            browse_your_collection: { kind: "command", value: "/prefix owned", label: "Browse your collection" }
        }
    }
});

pub static MASTER_CONTROL_CREDITS_GRANTED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "MASTER CONTROL: CREDITS GRANTED",
        body: r#"You have granted <white><bold>{{amount}}</bold></white> Data-Credits to the user <white><bold>{{target}}</bold></white>."#
    }
});

pub static ADMINISTRATIVE_GRANT: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ ADMINISTRATIVE GRANT",
        body: r#"An administrator has updated your account with <white><bold>{{amount}}</bold></white> Data-Credits."#,
        actions: {
            check_balance: { kind: "command", value: "/balance", label: "Check your new balance" }
        }
    }
});

pub static SQUAD_LINK_ESTABLISHED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ SQUAD LINK ESTABLISHED",
        body: r#"You have successfully joined the <white><bold>{{team}}</bold></white> squad."#,
        actions: {
            view_squad_roster: { kind: "command", value: "/team info", label: "View Squad Roster" }
        }
    }
});

pub static CONNECTION_ESTABLISHED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ CONNECTION ESTABLISHED",
        body: r#"<white><bold>{{username}}</bold></white> has linked with your squad.
    <dark_gray>»</dark_gray> Type <white>/tc</white> to welcome them."#
    }
});

pub static SQUAD_LINK_SEVERED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ SQUAD LINK SEVERED",
        body: r#"You have disconnected from the <white><bold>{{team}}</bold></white> squad.
    <dark_gray>»</dark_gray> You are now operating independently."#
    }
});

pub static USER_DISCONNECTED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ USER DISCONNECTED",
        body: r#"<white><bold>{{username}}</bold></white> has severed their link to the squad."#,
        actions: {
            view_updated_roster: { kind: "command", value: "/team info", label: "View updated roster" }
        }
    }
});

pub static SQUAD_ROSTER: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌐 SQUAD ROSTER",
        body: r#"Displaying roster for <white><bold>{{team_name}}</bold></white> (<white>{{member_count}}</white> members):
    {{roster_text}}
    <dark_gray>»</dark_gray> Use <white>/tc <message></white> for squad chat."#
    }
});

pub static BENGALI_KALA_JADU: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "JAABHAA ASSKIRIPT",
        body: "Jaabhaa asskripitsdgfrejhgijuhgyt",
    }
});