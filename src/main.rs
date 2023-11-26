use std::{collections::HashSet, fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use midir::MidiOutputConnection;
use midly::{live::LiveEvent, num::u7, MidiMessage};
use rdev::{listen, Event, EventType, Key};

use crate::{args::Args, config::Config};
mod args;
mod config;

const DEFAULT_CONFIG: &str = include_str!("../config.toml");

struct MakeyMidi {
    debug: bool,
    config: Config,
    output: MidiOutputConnection,
    pressed: HashSet<Key>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(args.config.unwrap_or_else(|| PathBuf::from("config.toml")))?;
    let output = args.midi.midi_device()?;

    let mut app = MakeyMidi {
        debug: args.debug,
        config,
        output,
        pressed: HashSet::new(),
    };

    if let Err(error) = listen(move |x| key_handler(&mut app, x)) {
        println!("[E] Rdev error: {:?}", error)
    }

    Ok(())
}

fn key_handler(app: &mut MakeyMidi, event: Event) {
    let event = match event.event_type {
        EventType::KeyPress(e) => {
            if !app.pressed.insert(e) {
                return;
            }

            let key = app.config.get_key(e);
            if key.is_some() && app.debug {
                println!("Key pressed: {:?}", e);
            }

            key.map(|x| MidiMessage::NoteOn {
                key: x.into(),
                vel: u7::max_value(),
            })
        }
        EventType::KeyRelease(e) => {
            app.pressed.remove(&e);

            let key = app.config.get_key(e);
            key.map(|x| MidiMessage::NoteOff {
                key: x.into(),
                vel: u7::max_value(),
            })
        }
        _ => None,
    };

    if let Some(event) = event {
        let mut buf = Vec::new();
        LiveEvent::Midi {
            channel: app.config.channel.into(),
            message: event,
        }
        .write(&mut buf)
        .unwrap();
        let _ = app.output.send(&buf);
    }
}

fn load_config(path: PathBuf) -> Result<Config> {
    let raw_config = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(error) => {
            println!("[E] Failed to read config file: {:?}", error);
            println!("[I] Using default config and writing to disk");
            if let Err(e) = fs::write("config.toml", DEFAULT_CONFIG) {
                println!("[E] Failed to write default config: {:?}", e);
            }
            DEFAULT_CONFIG.to_owned()
        }
    };
    Ok(toml::from_str(&raw_config)?)
}
