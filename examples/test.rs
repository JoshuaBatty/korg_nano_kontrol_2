extern crate korg_nano_kontrol_2;
extern crate midir;

fn main() {
    let midi_in = midir::MidiInput::new("Korg Nano Kontrol 2").unwrap();

    // A channel for sending events to the main thread.
    let (event_tx, event_rx) = std::sync::mpsc::channel();

    let mut inputs = Vec::new();

    // For each point used by the nano kontrol 2, check for events.
    for i in 0..midi_in.port_count() {
        let name = midi_in.port_name(i).unwrap();
        let event_tx = event_tx.clone();
        let midi_in = midir::MidiInput::new(&name).unwrap();
        let input = midi_in.connect(i, "nanoKONTROL2 SLIDER/KNOB", move |_stamp, msg, _| {
            if let Some(event) = korg_nano_kontrol_2::Event::from_midi(msg) {
                event_tx.send(event).unwrap();
            }
        }, ()).unwrap();
        inputs.push(input);
    }

    for event in event_rx {
        println!("{:?}", &event);
    }

    for input in inputs {
        input.close();
    }
}