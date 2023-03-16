extern crate keybd_event;
extern crate midir;
//#[cfg(target_os = "macos")]
//use std::thread::sleep;
//#[cfg(target_os = "macos")]
//use std::time::Duration;

/*use keybd_event::KeyboardKey::{KeySPACE, KeyBACKSPACE};
use keybd_event::KeyBondingInstance;
*/

use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::MidiInput;

/// String to look for when enumerating the MIDI devices
const PRESONUS_DEVICE: &str = "PreSonus FP2";

fn find_port<T>(midi_io: &T) -> Option<T::Port>
    where
        T: midir::MidiIO,
{
    let mut device_port: Option<T::Port> = None;
    for port in midi_io.ports() {
        if let Ok(port_name) = midi_io.port_name(&port) {
            if port_name.contains(PRESONUS_DEVICE) {
                device_port = Some(port);
                break;
            }
        }
    }
    device_port
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
    /*let mut kb = KeyBondingInstance::new().unwrap();
    #[cfg(target_os = "macos")]
    sleep(Duration::from_secs(1));
    kb.has_shift(true);
    kb.add_keys(&[KeySPACE]);
    kb.launching();*/
}
fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let midi_input = MidiInput::new("midir test output")?;
    let device_port = find_port(&midi_input);
    if device_port.is_none() {
        println!("Output device {} not found!", PRESONUS_DEVICE );
        return Err("Output device not found".into())
    }
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_input.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!("Choosing the only available input port: {}", midi_input.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_input.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };
    println!("\nOpening connection");
    let in_port_name = midi_input.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_input.connect(in_port, "midir-read-input", move |stamp, message, _| {
        println!("{}: {:?} (len = {})", stamp, message, message.len());
        match &message {
            &[176,16,1] => {
                println!("next");
            },
            &[176,16,65] => {
              println!("previous");
            },
            _ => {

            }
        }
    }, ())?;

    println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");

    Ok(())
}

