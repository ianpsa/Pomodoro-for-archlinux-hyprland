use std::process::Command;

pub struct AudioManager {}

impl AudioManager {
    pub fn are_headphones_connected() -> bool {
        let output = Command::new("pactl").arg("list").arg("sinks").output();

        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.to_lowercase().contains("headset")
                || stdout.to_lowercase().contains("buds")
                || stdout.to_lowercase().contains("headphones")
        } else {
            false
        }
    }
}
