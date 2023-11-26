use std::{collections::HashSet, fs, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use midir::MidiOutputConnection;
use midly::{live::LiveEvent, num::u7, MidiMessage};
use rdev::{listen, Event, EventType, Key};

use crate::{args::Args, config::Config};
mod args;
mod config;

struct MakeyMidi {
    debug: bool,
    config: Config,
    output: MidiOutputConnection,
    pressed: HashSet<Key>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let raw_config =
        fs::read_to_string(args.config.unwrap_or_else(|| PathBuf::from("config.toml")))?;
    let config = toml::from_str::<Config>(&raw_config)?;

    let mut app = MakeyMidi {
        debug: args.debug,
        config,
        output: args.midi.midi_device()?,
        pressed: HashSet::new(),
    };
    if let Err(error) = listen(move |x| callback(&mut app, x)) {
        println!("[E] Rdev error: {:?}", error)
    }

    Ok(())
}

fn callback(app: &mut MakeyMidi, event: Event) {
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
