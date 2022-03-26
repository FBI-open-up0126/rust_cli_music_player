use std::sync::PoisonError;

use clap::Args;
use log::{error, info};

use crate::util::settings::{PlaybackMode, SETTINGS};

/// You can change settings using this subcommand
#[derive(Args)]
pub struct ChangeSettings {
  /// Set the playback mode. They can be either of
  ///
  /// - Sequel (Play through the playlist and then exit)
  ///
  /// - LoopOnce (Play the same song forever)
  ///
  /// - LoopPlaylist (Loop the playlist forever)
  ///
  /// - Random (Randomize the playlist)
  #[clap(short, long)]
  playback_mode: Option<PlaybackMode>,

  /// Toggle weather to show settings or not. Default is false
  #[clap(short, long)]
  show_settings: bool,

  ///
  #[clap(short, long)]
  volume: Option<u8>,
}

impl ChangeSettings {
  pub fn handle(&mut self) {
    let mut is_settings_changed = false;
    let mut settings = SETTINGS.lock().unwrap_or_else(PoisonError::into_inner);

    if let Some(playback_mode) = self.playback_mode {
      settings.playback_mode = playback_mode;
      is_settings_changed = true;
    }
    if let Some(volume) = self.volume {
      settings.volume = volume;
      is_settings_changed = true;
    }

    if self.show_settings {
      info!("Playback mode --- {}", settings.playback_mode.to_string());
      info!("Volume --- {}", settings.volume);
    }

    if is_settings_changed {
      match settings.save() {
        Ok(_) => info!("Saved Successfully!"),
        Err(err) => error!(
          "Failed to save settings! No changes will be made! Error: {}",
          err
        ),
      }
    }
  }
}
