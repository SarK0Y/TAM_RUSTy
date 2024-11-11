use midly::{Header, Smf, Track, TrackEvent, TrackEventKind};
use midly::num::{u4, u7, u28};
pub fn mk_rnd_midi () {

} 
/*
use midly::{Header, Smf, Track, TrackEvent, TrackEventKind};
use midly::num::{u4, u7, u28};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new MIDI file
    let mut smf = Smf::new(Header::new(midly::Format::SingleTrack, u15::new(480)));

    // Create a new track
    let mut track = Track::new();

    // Add some notes
    let notes = [(60, 100), (62, 100), (64, 100), (65, 100), (67, 100)]; // C, D, E, F, G
    let mut time = u28::new(0);

    for (note, duration) in notes.iter() {
        // Note On
        track.push(TrackEvent {
            delta: time,
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new(*note),
                    vel: u7::new(64),
                },
            },
        });

        // Note Off (after duration)
        track.push(TrackEvent {
            delta: u28::new(*duration),
            kind: TrackEventKind::Midi {
                channel: u4::new(0),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new(*note),
                    vel: u7::new(64),
                },
            },
        });

        time = u28::new(0); // Reset delta time for next note
    }

    // Add End of Track event
    track.push(TrackEvent {
        delta: u28::new(0),
        kind: TrackEventKind::Meta(midly::MetaMessage::EndOfTrack),
    });

    // Add the track to the MIDI file
    smf.tracks.push(track);

    // Write the MIDI file
    std::fs::write("output.mid", smf.to_bytes()?)?;

    Ok(())
}
*/