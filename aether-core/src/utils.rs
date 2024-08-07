use azalea::chat::ChatPacket;
use azalea::ecs::prelude::*;
use regex::Regex;

#[deprecated]
#[macro_export]
macro_rules! msg {
    ($client: expr, $username: expr, $message: expr) => {
        $client.send_command_packet(format!("w {} {}", $username, $message).as_str())
    };
}

/// Parses Chat Content (This function is adapted and can parse whisper message on 2b2t.org)
pub fn parse_chat_content(chat: &ChatPacket) -> (Option<String>, String, bool) {
    let (username, content) = chat.split_sender_and_content();
    // If the username is present, use split_sender_and_content
    if let Some(uname) = username {
        // Return the result of split_sender_and_content as it's not a whisper
        return (Some(uname), content, chat.is_whisper());
    }

    // Define regex pattern to match whispers
    let whisper_pattern = Regex::new(r"(?P<uname>\w+) whispers: (?P<message>.+)").unwrap();

    // If the content matches the whisper pattern
    if let Some(captures) = whisper_pattern.captures(&content) {
        let uname = captures.name("uname").map(|u| u.as_str().to_string());
        let message = captures
            .name("message")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        return (uname, message, true); // It's a whisper
    }

    // If no match is found, return None for the username and the original content
    (None, content, false)
}

/// This component is present when the bot has passed the queue and is in the world
#[derive(Component)]
pub struct InWorld;
