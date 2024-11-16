use midly::{Header, Smf, Track, TrackEvent, TrackEventKind, Timing::Metrical};
use midly::num::{u4, u7, u28, u15};
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u8 as u8__;
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u32 as u32__;
use Mademoiselle_Entropia::true_rnd::UID_UTF8 as mk_uid;
use crate::custom_traits::STRN;
use crate::errMsg0;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub fn mk_rnd_midi (duration: u16) {
    let duration_u15 = u15::new( duration );
     let mut smf = Smf::new(Header::new(midly::Format::SingleTrack,  Metrical ( duration_u15 ) ) );

    // Create a new track
    let mut track = Track::new();
    let mut time = u28::new(0);
    let mut notes: Vec < (u8, u32, u8, u8) > = Vec::new ();
    let mut already_gen_time: u16 = 0;
    let mut num_of_channel: u8 = 4;
    let mut note: u8 = 23;
    let mut vel_: u8 = 64;
    let mut duration_: u32 = 67;
    while already_gen_time < duration {
        vel_ = u8__( Some( vel_ ) );
        duration_ = u32__( ) % 999;
        note = u8__( Some( note ) ) % 108;
        if note < 21 && (note & 1 ) == 1  { note += 21 }
        if note < 21 && (note & 1 ) == 0  { note += 23 }
        num_of_channel = u8__( Some ( num_of_channel ) ) % 16;
        already_gen_time += 1;
        notes.push ( (note, duration_, vel_, num_of_channel ) );
    }
    dbg! (&already_gen_time);
   // errMsg0( "");
    for i in notes {
        note = i.0;
        dbg! (&note);
        duration_ = i.1;
        vel_ = i.2;
        num_of_channel = i.3;
        // Note On
        track.push(TrackEvent {
            delta: time,
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new( note ),
                    vel: u7::new( vel_ ),
                },
            },
        });

        // Note Off (after duration)
        track.push(TrackEvent {
            delta: u28::new( duration_ ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new( note ),
                    vel: u7::new( vel_),
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
    let file_name = format! ( "Universum Vox.{}.mid", mk_uid( 24 ));
    let full_path = format! ( "{}/{file_name}", crate::cmd_keys::midi_dir ( None ) );
    smf.save( &full_path );
    let msg = format! ("Dear User, data was written to {full_path}\nPlease, hit any key to continue.. Thanks.");
    errMsg0( &msg );


}
pub fn mk_rnd_midi_advanced (duration: u16) {
    let duration_u15 = u15::new( duration );
     let mut smf = Smf::new(Header::new(midly::Format::SingleTrack,  Metrical ( duration_u15 ) ) );

    // Create a new track
    let mut track = Track::new();
    let mut time = u28::new(0);
    let mut notes: Vec < (u8, u32, u8, u8) > = Vec::new ();
    let mut already_gen_time: u16 = 0;
    let mut num_of_channel: u8 = 4;
    let mut note: u8 = 23;
    let mut vel_: u8 = 64;
    let mut duration_: u32 = 67;
    while already_gen_time < duration {
        vel_ = u8__( Some( vel_ ) );
        duration_ = u32__( ) % 999;
        note = u8__( Some( note ) ) % 108;
        if note < 21 && (note & 1 ) == 1  { note += 21 }
        if note < 21 && (note & 1 ) == 0  { note += 23 }
        num_of_channel = u8__( Some ( num_of_channel ) ) % 16;
        already_gen_time += 1;
        notes.push ( (note, duration_, vel_, num_of_channel ) );
    }
    dbg! (&already_gen_time);
   // errMsg0( "");
    for i in notes {
        note = i.0;
        dbg! (&note);
        duration_ = i.1;
        vel_ = i.2;
        num_of_channel = i.3;
        // Note On
        track.push(TrackEvent {
            delta: time,
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new( note ),
                    vel: u7::new( vel_ ),
                },
            },
        });

        // Note Off (after duration)
        track.push(TrackEvent {
            delta: u28::new( duration_ ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new( note ),
                    vel: u7::new( vel_),
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
    let file_name = format! ( "Universum Vox.{}.mid", mk_uid( 24 ));
    let full_path = format! ( "{}/{file_name}", crate::cmd_keys::midi_dir ( None ) );
    smf.save( &full_path );
    let msg = format! ("Dear User, data was written to {full_path}\nPlease, hit any key to continue.. Thanks.");
    errMsg0( &msg );


}
pub fn universum_vox (cmd: &String) {
    let cmd = cmd.replace ("universum vox", "").trim_end().trim_start ().strn ();
    let mut duration = 15u16;
    if let Ok ( x ) = cmd.parse:: <u16> () { duration = x }
    mk_rnd_midi(duration)
} 
pub fn load_uv_conf <P: AsRef<Path> >(path: P) -> Result<crate::enums::universum_vox_note, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let uv_note: crate::universum_vox_note = serde_json::from_reader(reader)?; 
    Ok (uv_note )
}
/*
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct User {
    fingerprint: String,
    location: String,
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

fn main() {
    let u = read_user_from_file("test.json").unwrap();
    println!("{:#?}", u);
}
----------
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