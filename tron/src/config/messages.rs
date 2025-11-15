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
        body: r#"You currently have no active connections.
        Make friends to grow your <gradient:#6A00A3:#B200FF>network</gradient>!"#
    }
});

pub static FRIEND_NETWORK: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌐 FRIEND NETWORK",
        body: r#"Displaying <white>{{count}}</white> friend{{s}}:
       {{friend_list}}"#
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

pub static REPORT_PLAYER: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✅ REPORT SUBMITTED",
        body: "
        Your report against <white><bold>{{target}}</bold></white> has been logged for review.\n\
        <dark_gray>Reason:</dark_gray> <white>{{reason}}</white>\n\
        <dark_gray>»</dark_gray> <light_purple>Thank you for helping keep the network secure.</light_purple>
        "
    }
});

pub static INVALID_REPORT_TARGET: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ INVALID TARGET",
        body: "You cannot submit a report against your own user profile."
    }
});

pub static TARGET_NOT_FOUND: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ PLAYER NOT FOUND",
        body: "The player <white><bold>{{username}}</bold></white> could not be located on the network. Either the player isn't active or not found in database."
    }
});

pub static PLAYER_OFFLINE: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ PLAYER IS OFFLINE",
        body: "The player <white><bold>{{username}}</bold></white> left the network."
    }
});

pub static PLAYER_REPORT_NOTIFICATION: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚡ NEW PLAYER REPORT",
        body: "
        Player <white><bold>{{username}}</bold></white> has reported <white><bold>{{target}}</bold></white>.
        ",
        actions: {
            view_reports: { kind: "command", value: "/admin report list", label: "View reports" }
        }
    }
});

pub static BENGALI_KALA_JADU: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "JAABHAA ASSKIRIPT",
        body: "Jaabhaa asskripitsdgfrejhgijuhgyt",
    }
});

pub static NOT_SUBSCRIBED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ NOT SUBSCRIBED",
        body: "You are not subscribed to any channel",
        actions: {
            global: {kind: "command", value: "/global", label: "Join Global"},
            hindi: {kind: "command", value: "/hindi", label: "Join Hindi"}
        }
    }
});

pub static NOT_A_FRIEND: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ NOT A FRIEND",
        body: "<white><bold>{{target}}</bold></white> is not your friend.",
        actions: {
            send: {kind: "command", value: "/friend {{target}}", label: "Send Friend Request"},
        }
    }
});

pub static FAILED_TO_SEND_WHISPER: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ FAILED TO SEND WHISPER",
        body: "Failed to send whisper to <white><bold>{{target}}</bold></white>. Please try again later if occurred repeatedly report bug.",
        actions: {
            bug: {kind: "command", value: "/bug failed to send whisper to {{target}}", label: "Report Bug"},
        }
    }
});

pub static GENERIC_STATUS_ERROR: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "error",
        title: "❌ {{code}}",
        body: "{{message}}",
        actions: {
            bug: {kind: "command", value: "/bug [{{type}}] {{message}}", label: "Report Bug"},
        }
    }
});

pub static NO_BUGS_FOUND: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🐞 BUG REPORTS",
        body: r#"No bugs found in the system. Everything looks <green>clean</green>!"#
    }
});

pub static BUG_LIST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🐛 REPORTED BUGS",
        body: r#"Displaying <white>{{count}}</white> reported issue{{s}}:
    {{list}}"#
    }
});

pub static BUG_DETAIL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔎 BUG DETAIL VIEW",
        body: r#"
<gray>Bug ID:</gray> <yellow><bold>#{{id}}</bold></yellow>
<gray>Reporter:</gray> <aqua>{{reporter}}</aqua>
<gray>Reported:</gray> <white>{{created}}</white>

<gray>Description:</gray>
<light_purple>{{description}}</light_purple>

<dark_gray>──────────────────────────────</dark_gray>
[<red><click:run_command:'/admin bug delete {{id}}'>Delete Bug</click></red>]
"#
    }
});

pub static NO_REPORTS_FOUND: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "📋 REPORTS",
        body: "No reports found in the network. Everything looks <green>clean</green>!"
    }
});

pub static ENABLE_SCOREBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✓ SUCCESSFULLY ENABLED SCOREBOARD",
        body: "You enabled your scoreboard. Enjoy live stats and let us know if found any bug!",
        actions: {
           disable: {kind: "command", value: "/toggle", label: "Disable Scoreboard"},
        }
    }
});

pub static DISABLE_SCOREBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "✓ SUCCESSFULLY DISABLED SCOREBOARD",
        body: "You disabled your scoreboard. Enjoy true vanilla experience and toggle the scoreboard whenever you want!",
        actions: {
           enable: {kind: "command", value: "/toggle", label: "Enable Scoreboard"},
        }
    }
});

pub static REPORT_LIST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "📋 REPORTED PLAYERS",
        body:
        "Displaying <white>{{count}}</white> reported player{{s}}:\n\
        {{list}}"
    }
});

pub static REPORT_DETAIL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔎 REPORT DETAIL VIEW",
        body: "
    <gray>Report ID:</gray> <yellow><bold>#{{id}}</bold></yellow>\n\
    <gray>Reporter:</gray> <aqua>{{reporter}}</aqua>\n\
    <gray>Player:</gray> <aqua>{{player}}</aqua>\n\
    <gray>Reported:</gray> <white>{{created}}</white>\n\
    <gray>Reason:</gray>\n\
    \n\
    <light_purple>{{report}}</light_purple>\n\
    \n\
    <dark_gray>──────────────────────────────</dark_gray>\n\
    [<red><click:run_command:'/admin bug delete {{id}}'>Delete Bug</click></red>]
    "
    }
});

pub static NO_SERVERS_FOUND: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌐 SERVERS",
        body: r#"No servers found in the system."#
    }
});

pub static SERVER_LIST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌐 SERVERS",
        body:
        "Displaying <white>{{count}}</white> server{{s}}:\n\
        {{list}}"
    }
});

pub static SERVER_DETAIL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔎 SERVER DETAILS",
        body: "
    <gray>SERVER ID:</gray> <yellow><bold>#{{id}}</bold></yellow>\n\
    <gray>Name:</gray> <aqua>{{name}}</aqua>\n\
    <gray>Description:</gray> <aqua>{{description}}</aqua>\n\
    <gray>Created By:</gray> <aqua>{{creator}}</aqua>\n\
    <gray>Created On:</gray> <white>{{created}}</white>\n\
    <gray>Address:</gray> <white>{{address}}</white>\n\
    <dark_gray>──────────────────────────────</dark_gray>\n\
    [<red><click:run_command:'/admin server delete {{name}}'>Delete Server</click></red>]\n\
    [<red><click:run_command:'/admin set landing {{name}}'>Make Landing</click></red>]
    "
    }
});

pub static EXIT_CHAT: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "➜] SUCCESSFULLY EXITED CHANNEL",
        body: "You exited channel successfully. Join one to continue chatting!",
        actions: {
          global: {kind: "command", value: "/global", label: "Global Channel"},
          hindi: {kind: "command", value: "/hindi", label: "Hindi Channel"},
        }
    }
});

pub static GLOBAL_CHAT: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🌍 SUCCESSFULLY JOINED GLOBAL CHANNEL",
        body: "You entered the global channel. Follow the rules for global chat and be respectful talking to other players!\n
              <dark_gray>»</dark_gray> 1. Don't abuse or say anything offensive\n
              <dark_gray>»</dark_gray> 2. Only english is allowed\n
              <dark_gray>»</dark_gray> 3. Be respectful and don't spam\n
              <dark_gray>»</dark_gray> 4. Don't promote or advertise other servers\n
              <dark_gray>»</dark_gray> 5. Report players if they try to bully or abuse\n",
        actions: {
          exit: {kind: "command", value: "/exit", label: "Exit Channel"},
          hindi: {kind: "command", value: "/hindi", label: "Join Hindi Channel"},
        }
    }
});

pub static HINDI_CHANNEL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🇮🇳 SUCCESSFULLY JOINED HINDI CHANNEL",
        body: "Aapne hindi channel join kiya hai diye gaye rules ka dhyad rakhte hue dusre players se baat kare!\n
              <dark_gray>»</dark_gray> 1. Gali ya abhadra bhasha ka upyog na kare\n
              <dark_gray>»</dark_gray> 2. Hindi bhasha ka upyog hi kare\n
              <dark_gray>»</dark_gray> 3. Dusre players ke sath ijat se baat kare\n
              <dark_gray>»</dark_gray> 4. Dusre servers promote na kare\n
              <dark_gray>»</dark_gray> 5. Agar kisine apko pareshan ya gali dene ki kosish kari to turant report kare\n",
        actions: {
          exit: {kind: "command", value: "/exit", label: "Exit Channel"},
          hindi: {kind: "command", value: "/global", label: "Join Global Channel"},
        }
    }
});

pub static DM_CHANNEL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔗 SUCCESSFULLY JOINED DM CHANNEL WITH {{friend}}",
        body: "You have joined the DM channel with {{friend}}. Your chat your rules but keep in mind you can still report player if they try to abuse or bully you.",
        actions: {
          exit: {kind: "command", value: "/exit", label: "Exit Channel"},
        }
    }
});

pub static JOINED_CHANNEL: Lazy<Template<'static>> = Lazy::new(|| {
    Template::new("<gradient:#B200FF:#6A00A3>╰┈➤ {{username}} joined the channel</gradient>")
});

pub static FRIEND_CHAT_REQUEST_SENT: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "📨 SENT A FRIEND CHAT REQUEST TO {{friend}}",
        body: "You had sent a friend chat request to {{friend}}. If they accept, you will be able to chat privately.",
    }
});

pub static FRIEND_CHAT_REQUEST_RECEIVED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "📨 RECEIVED A FRIEND CHAT REQUEST FROM {{friend}}",
        body: "You had received a friend chat request from {{friend}}. If you accept, you will be able to chat privately.",
        actions: {
            accept: {kind: "command", value: "/dmaccept {{token}}", label: "Accept"},
        }
    }
});

pub static FRIEND_CHAT_JOINED: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "💬 JOINED FRIEND CHAT WITH {{friend}}",
        body: "You joined friend chat with {{friend}}. Your chat your rules but keep in mind you can still report player if they try to abuse or bully you.",
        actions: {
            accept: {kind: "command", value: "/exit", label: "Exit Channel"},
        }
    }
});

pub static COINS_LEADERBOARD_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "💰 COINS LEADERBOARD",
        body: "No players found in the coins leaderboard yet."
    }
});

pub static COINS_LEADERBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "💰 COINS LEADERBOARD",
        body: "
<white><bold>Top Players:</bold></white>

{{list}}

<dark_gray>────────────────────────</dark_gray>
<gray>Your Rank:</gray> <yellow><bold>{{rank}}</bold></yellow>
        "
    }
});

pub static DEATHS_LEADERBOARD_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "☠️ DEATHS LEADERBOARD",
        body: "No players found in the deaths leaderboard yet."
    }
});

pub static DEATHS_LEADERBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "☠️ DEATHS LEADERBOARD",
        body: "
<white><bold>Top Players:</bold></white>

{{list}}

<dark_gray>────────────────────────</dark_gray>
<gray>Your Rank:</gray> <yellow><bold>{{rank}}</bold></yellow>
        "
    }
});

pub static KILLS_LEADERBOARD_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚔️ KILLS LEADERBOARD",
        body: "No players found in the kills leaderboard yet."
    }
});

pub static KILLS_LEADERBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⚔️ KILLS LEADERBOARD",
        body: "
<white><bold>Top Players:</bold></white>

{{list}}

<dark_gray>────────────────────────</dark_gray>
<gray>Your Rank:</gray> <yellow><bold>{{rank}}</bold></yellow>
        "
    }
});

pub static OVERALL_LEADERBOARD_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🏅 OVERALL LEADERBOARD",
        body: "No players found in the overall leaderboard yet."
    }
});

pub static OVERALL_LEADERBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🏅 OVERALL LEADERBOARD",
        body: "
<white><bold>Top Players:</bold></white>

{{list}}

<dark_gray>────────────────────────</dark_gray>
<gray>Your Rank:</gray> <yellow><bold>{{rank}}</bold></yellow>
        "
    }
});

pub static KD_LEADERBOARD_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🛡️ KD LEADERBOARD",
        body: "No players found in the overall leaderboard yet."
    }
});

pub static KD_LEADERBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🛡️ KD LEADERBOARD",
        body: "
<white><bold>Top Players:</bold></white>

{{list}}

<dark_gray>────────────────────────</dark_gray>
<gray>Your Rank:</gray> <yellow><bold>{{rank}}</bold></yellow>
        "
    }
});

pub static TEAMS_LEADERBOARD_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🏆 TEAMS LEADERBOARD",
        body: "No team found in the teams leaderboard yet."
    }
});

pub static TEAMS_LEADERBOARD: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🏆 TEAMS LEADERBOARD",
        body: "
<white><bold>Top Teams:</bold></white>

{{list}}

<dark_gray>────────────────────────</dark_gray>
<gray>Your Team's Rank:</gray> <yellow><bold>{{rank}}</bold></yellow>
        "
    }
});

pub static ADMIN_LIST_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔑 EMPTY ADMIN LIST",
        body: "No admins found in the admin list yet."
    }
});

pub static ADMIN_LIST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "success",
        title: "🛡️ KD LEADERBOARD",
        body: "
<white><bold>ADMINS:</bold></white>

{{list}}
        "
    }
});

pub static ADMIN_DETAIL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔎 ADMIN DETAILS",
        body: "
    <gray>ADMIN ID:</gray> <yellow><bold>#{{id}}</bold></yellow>\n\
    <gray>Username:</gray> <aqua>{{username}}</aqua>\n\
    <dark_gray>──────────────────────────────</dark_gray>\n\
    [<red><click:run_command:'/admin perms demote {{username}}'>Demote</click></red>]\n\
    "
    }
});

pub static MODERATOR_LIST_EMPTY: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "👤 EMPTY MODERATOR LIST",
        body: "No moderators found in the moderator list yet."
    }
});

pub static MODERATOR_LIST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "success",
        title: "👤 MODERATOR LIST",
        body: "
<white><bold>MODERATORS:</bold></white>

{{list}}
        "
    }
});

pub static MODERATOR_DETAIL: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "🔎 MODERATOR DETAILS",
        body: "
    <gray>MODERATOR ID:</gray> <yellow><bold>#{{id}}</bold></yellow>\n\
    <gray>Username:</gray> <aqua>{{username}}</aqua>\n\
    <dark_gray>──────────────────────────────</dark_gray>\n\
    [<red><click:run_command:'/admin perms demote {{username}}'>Demote</click></red>]\n\
    "
    }
});

pub static EMPTY_REDEEM_CODES: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "info",
        title: "⌨️ EMPTY REDEEM CODES LIST",
        body: "No redeem codes found in the redeem codes list yet."
    }
});

pub static REDEEM_LIST: Lazy<Template<'static>> = Lazy::new(|| {
    message! {
        type: "success",
        title: "⌨️ REDEEM LIST",
        body: "
<white><bold>REDEEM CODES:</bold></white>

{{list}}
        "
    }
});
