use std::{path::PathBuf, process};

use anyhow::{Context, Result};
use clap::Parser;
use midir::{MidiOutput, MidiOutputConnection};

#[derive(Parser)]
pub struct Args {
    /// The location of the config file.
    /// Defaults to `config.toml` in the current directory.
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// Logs each key event to stdout.
    #[clap(short, long)]
    pub debug: bool,

    #[clap(subcommand)]
    pub midi: Midi,
}

#[derive(Parser)]
pub enum Midi {
    /// Creates a new virtual MIDI output device.
    /// Only works on Linux.
    #[clap(name = "create")]
    #[cfg(target_os = "linux")]
    Create,

    /// Connects to a MIDI device to send events to.
    #[clap(name = "connect")]
    Connect {
        /// The name of the MIDI device to connect to (uses string similarity).
        /// If not specified, the first device found will be used.
        name: Option<String>,
    },

    /// Lists all available MIDI devices.
    /// For use with the `connect` subcommand.
    List,
}

impl Midi {
    pub fn midi_device(&self) -> Result<MidiOutputConnection> {
        Ok(match self {
            #[cfg(target_os = "linux")]
            Midi::Create => MidiOutput::new("makey-midi output")?
                .create_virtual("makey-midi output")
                .unwrap(),
            Midi::Connect { name } => {
                let input = MidiOutput::new("makey-midi output")?;
                let ports = input.ports();

                let port = match name {
                    None => ports.into_iter().next().context("No ports found")?,
                    Some(name) => {
                        let mut best = (None, 0.0);
                        for port in ports {
                            let port_name = input.port_name(&port)?;
                            let similarity = strsim::sorensen_dice(&port_name, name);
                            if similarity > best.1 {
                                best = (Some(port), similarity);
                            }
                        }
                        best.0.context("No ports found")?
                    }
                };
                println!("Connecting to port {}", input.port_name(&port)?);
                input.connect(&port, "makey-midi output")?
            }
            Midi::List => {
                let input = MidiOutput::new("42synth input")?;
                println!("Found {} MIDI devices:", input.ports().len());
                for port in input.ports() {
                    let port_name = input.port_name(&port)?;
                    println!(" - {port_name}");
                }
                process::exit(0);
            }
        })
    }
}
