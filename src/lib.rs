//! A simple crate for converting the MIDI output of the Korg nano KONTROL 2 into user-friendly
//! rust-esque types.

/// The name of the ports on which the `nano kontrol 2` emits MIDI input values.
pub const MIDI_INPUT_PORT_PREFIX: &'static str = "nanoKONTROL2 SLIDER/KNOB";

/// The top two track buttons.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum TrackButton {
    Left,
    Right,
}

/// The three marker buttons.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum MarkerButton {
    Set,
    Left,
    Right,
}

/// The five transport buttons.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Transport {
    Rewind,
    Fastforward,
    Stop,
    Play,
    Record,
}

/// The 3 distinct rows on which `Button`s are placed.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ButtonRow {
    Solo,
    Mute,
    Record,
}

/// Events emitted from button presses.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum State {
    /// The button was pressed.
    On,
    /// The button was released.
    Off
}

/// The controls on the nano kontrol 2 is a strip of 8.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Strip { A, B, C, D, E, F, G, H }

/// Controller events.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Event {

    /// The value to which the slider was set.
    ///
    /// Values range from `0` to `127` (inclusive).
    RotarySlider(Strip, u8),

    /// The value to which the slider was set.
    ///
    /// Values range from `0` to `127` (inclusive).
    VerticalSlider(Strip, u8),

    /// A button was pressed on the given row.
    Button(ButtonRow, Strip, State),

    /// The two buttons on the upper left hand side of the controller.
    TrackButton(TrackButton, State),

    /// The single cycle button on the 2nd row and left hand side of the controller.
    CycleButton(State),

    /// The three maker buttons on the 2nd row and left hand side of the controller..
    MarkerButton(MarkerButton, State),

    /// Media playback-style control buttons.
    TransportButton(Transport, State),
}


impl Strip {
    /// Create an `Strip` from the given value where 0 == A, 1 == B, etc.
    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Strip::A),
            1 => Some(Strip::B),
            2 => Some(Strip::C),
            3 => Some(Strip::D),
            4 => Some(Strip::E),
            5 => Some(Strip::F),
            6 => Some(Strip::G),
            7 => Some(Strip::H),
            _ => None,
        }
    }
}

impl Event {

    /// Produce an `Event` from the given MIDI message itself.
    pub fn from_midi(msg: &[u8]) -> Option<Self> {
        // Receive control events.
        match msg.len() {
            3 => match (msg[0], msg[1], msg[2]) {

                // Rotary sliders.
                (176, n @ 16..=23, value) => {
                    let strip = Strip::from_u8(n - 16).unwrap();
                    Some(Event::RotarySlider(strip, value).into())
                },

                // Vertical sliders.
                (176, n @ 0..=7, value) => {
                    let strip = Strip::from_u8(n).unwrap();
                    Some(Event::VerticalSlider(strip, value).into())
                },

                ///////////////////
                ///// Buttons /////
                ///////////////////

                // Track buttons.
                (176, n @ 58..=59, state) => {
                    let button = match n {
                        58 => TrackButton::Left,
                        59 => TrackButton::Right,
                        _ => unreachable!(),
                    };
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::TrackButton(button, state).into())
                },

                // Cycle button.
                (176, 99, state) => {
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::CycleButton(state).into())
                },

                // Marker buttons.
                (176, n @ 60..=62, state) => {
                    let button = match n {
                        60 => MarkerButton::Set,
                        61 => MarkerButton::Left,
                        62 => MarkerButton::Right,
                        _ => unreachable!(),
                    };
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::MarkerButton(button, state).into())
                },

                // Transport buttons.
                (176, n @ 44..=47, state) => {
                    let transport = match n {
                        44 => {
                            if state == 0 { Transport::Rewind } else { Transport::Fastforward }
                        },
                        45 => Transport::Stop,
                        46 => Transport::Play,
                        47 => Transport::Record,
                        _ => unreachable!(),
                    };
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::TransportButton(transport, state).into())
                },

                // Top solo row buttons.
                (176, n @ 32..=39, state) => {
                    let strip = Strip::from_u8(n - 32).unwrap();
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::Button(ButtonRow::Solo, strip, state).into())
                },

                // Middle mute row buttons.
                (176, n @ 48..=55, state) => {
                    let strip = Strip::from_u8(n - 48).unwrap();
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::Button(ButtonRow::Mute, strip, state).into())
                },

                // Bottom record row buttons.
                (176, n @ 64..=71, state) => {
                    let strip = Strip::from_u8(n - 64).unwrap();
                    let state = if state == 0 { State::Off } else { State::On };
                    Some(Event::Button(ButtonRow::Record, strip, state).into())
                },

                _ => None,

            },
            _ => None,
        }
    }
        
    

}