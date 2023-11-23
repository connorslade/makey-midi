use std::{collections::HashSet, fs};

use anyhow::Result;
use midir::{os::unix::VirtualOutput, MidiOutput, MidiOutputConnection};
use midly::{live::LiveEvent, num::u7, MidiMessage};
use rdev::{listen, Event, EventType, Key};

use crate::config::Config;

mod config;

struct MakeyMidi {
    config: Config,
    output: MidiOutputConnection,
    pressed: HashSet<Key>,
}

fn main() -> Result<()> {
    let raw_config = fs::read_to_string("config.toml")?;
    let config = toml::from_str::<Config>(&raw_config)?;

    let output = MidiOutput::new("makey-midi output")?
        .create_virtual("makey-midi output")
        .unwrap();

    let mut app = MakeyMidi {
        config,
        output,
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

            let key = app.config.midi.get_key(e);
            key.map(|x| MidiMessage::NoteOn {
                key: x.into(),
                vel: u7::max_value(),
            })
        }
        EventType::KeyRelease(e) => {
            app.pressed.remove(&e);

            let key = app.config.midi.get_key(e);
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
            channel: app.config.midi.channel.into(),
            message: event,
        }
        .write(&mut buf)
        .unwrap();
        let _ = app.output.send(&buf);
    }
}
