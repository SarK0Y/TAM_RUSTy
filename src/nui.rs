use midly::{Header, Smf, Track, TrackEvent, TrackEventKind, Timing::Metrical};
use midly::num::{u4, u7, u28, u15};
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u8 as u8__;
use Mademoiselle_Entropia::true_rnd::get_true_rnd_u32 as u32__;
use Mademoiselle_Entropia::true_rnd::UID_UTF8 as mk_uid;
use crate::custom_traits::STRN;
use crate::{errMsg0, getkey};
use serde::{Deserialize, Serialize, Serializer};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
pub fn mk_rnd_midi_advanced (duration: u16, path_to_conf: &String) {
    let duration_u15 = u15::new( duration );
     let mut smf = Smf::new(Header::new(midly::Format::SingleTrack,  Metrical ( duration_u15 ) ) );

    // Create a new track
    let mut track = Track::new();
    let mut uv_note: crate::enums::universum_vox_note =
                      match load_uv_conf( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"alg0\":1,\n
            \"num_of_channels\":16,\n
            \"duration\":150,\n
            \"const_duration\":false,\n
            \"deviate_duration\":0,\n
            \"velocity_level\":211,\n
            \"const_velocity\":true,\n
            \"range\":null,\n
            \"bottom\":17,\n
            \"arr\":[63,78]\n-----\nRemark: max range of notes is [0..128]
} "); return;} };
    let mut time = u28::new(0);
    let mut notes: Vec < (u8, u32, u8, u8) > = Vec::new ();
    let mut already_gen_time: u16 = 0;
    let mut num_of_channel: u8 = uv_note.num_of_channels;
    let mut range_of_notes: u8 = 1;
    if uv_note.range.is_none () && uv_note.bottom.is_none () && uv_note.arr.is_none () {
        range_of_notes = 128;
    }
    let mut bottom_note: u8 = 0;
    let mut len_arr = 0usize;
    if uv_note.arr.is_some () { len_arr = uv_note.arr.as_ref().unwrap().len (); } else {
        if let Some ( x ) = uv_note.bottom { bottom_note = x }
        if let Some ( x ) = uv_note.range { range_of_notes = x }
        if bottom_note + range_of_notes > 127 {
            let dt = bottom_note + range_of_notes - 127;
            range_of_notes -= dt;
        }
    }
    let mut note: u8 = 23;
    let mut note_: u8 = 23;
    let mut vel_: u8 = 64;
    let mut duration_: u32 = 67;
    while already_gen_time < duration {
        if uv_note.const_velocity { vel_ = uv_note.velocity_level } else {
            vel_ = u8__( Some( vel_ ) ) % uv_note.velocity_level;
        }
        if uv_note.const_duration { 
            duration_ = uv_note.duration;
            if uv_note.deviate_duration > 0 { duration_ += u32__( ) % uv_note.deviate_duration; }
         } else {
            duration_ = u32__( ) % uv_note.duration;
        }
        note = u8__( Some( note ) );
        if len_arr > 0 {
            note_ = note %  len_arr as u8;
            note_ = uv_note.arr.as_mut().unwrap() [ note_ as usize ]; 
        } else {
            note_ = note %  range_of_notes;
            note_ = note_ + bottom_note;
        }
        num_of_channel = u8__( Some ( num_of_channel ) ) % uv_note.num_of_channels;
        already_gen_time += 1;
        notes.push ( (note_, duration_, vel_, num_of_channel ) );
    }
    dbg! (&already_gen_time);
   // errMsg0( "");
   let mut dt = 0u32;
    for i in notes {
        note = i.0;
        dbg! (&note);
        duration_ = i.1;
        vel_ = i.2;
        num_of_channel = i.3;
        // Note On
        track.push(TrackEvent {
            delta: u28::new( dt ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new( note ),
                    vel: u7::new( vel_ ),
                },
            },
        });

        // Note Off (after duration)
        dt += duration_;
        track.push(TrackEvent {
            delta: u28::new( dt ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new( note ),
                    vel: u7::new( vel_),
                },
            },
        });

        dt = 0; // Reset delta time for next note
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
pub fn mk_rnd_midi_dense (duration: u16, path_to_conf: &String) {
    let duration_u15 = u15::new( duration );
     let mut smf = Smf::new(Header::new(midly::Format::SingleTrack,  Metrical ( duration_u15 ) ) );

    // Create a new track
    let mut track = Track::new();
    let mut uv_note: crate::enums::universum_vox_note =
                      match load_uv_conf( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"alg0\":1,\n
            \"num_of_channels\":16,\n
            \"duration\":150,\n
            \"const_duration\":false,\n
            \"deviate_duration\":0,\n
            \"velocity_level\":211,\n
            \"const_velocity\":true,\n
            \"range\":null,\n
            \"bottom\":17,\n
            \"arr\":[63,78]\n-----\nRemark: max range of notes is [0..128]
} "); return;} };
    let mut time = u28::new(0);
    let mut notes: Vec < (u8, u32, u8, u8) > = Vec::new ();
    let mut already_gen_time: u16 = 0;
    let mut num_of_channel: u8 = uv_note.num_of_channels;
    let mut range_of_notes: u8 = 1;
    if uv_note.range.is_none () && uv_note.bottom.is_none () && uv_note.arr.is_none () {
        range_of_notes = 128;
    }
    let mut bottom_note: u8 = 0;
    let mut len_arr = 0usize;
    if uv_note.arr.is_some () { len_arr = uv_note.arr.as_ref().unwrap().len (); } else {
        if let Some ( x ) = uv_note.bottom { bottom_note = x }
        if let Some ( x ) = uv_note.range { range_of_notes = x }
        if bottom_note + range_of_notes > 127 {
            let dt = bottom_note + range_of_notes - 127;
            range_of_notes -= dt;
        }
    }
    let mut note: u8 = 23;
    let mut note_: u8 = 23;
    let mut vel_: u8 = 64;
    let mut duration_: u32 = 67;
    let notes_per_channel = notes.len() / uv_note.num_of_channels as usize;
   let mut count_notes_per_ch = 0usize;
   num_of_channel = 0;
    while already_gen_time < duration {
        if uv_note.const_velocity { vel_ = uv_note.velocity_level } else {
            vel_ = u8__( Some( vel_ ) ) % uv_note.velocity_level;
        }
        if uv_note.const_duration { 
            duration_ = uv_note.duration;
            if uv_note.deviate_duration > 0 { duration_ += u32__( ) % uv_note.deviate_duration; }
         } else {
            duration_ = u32__( ) % uv_note.duration;
        }
        note = u8__( Some( note ) );
        if len_arr > 0 {
            note_ = note %  len_arr as u8;
            note_ = uv_note.arr.as_mut().unwrap() [ note_ as usize ]; 
        } else {
            note_ = note %  range_of_notes;
            note_ = note_ + bottom_note;
        }
        if count_notes_per_ch == notes_per_channel { num_of_channel += 1; count_notes_per_ch = 0;}
        else { count_notes_per_ch += 1; }
        already_gen_time += 1;
        notes.push ( (note_, duration_, vel_, num_of_channel ) );
    }
    
    dbg! (&already_gen_time);
   // errMsg0( "");
   let mut dt = 0u32;
   num_of_channel = 0;
    for i in notes {
        note = i.0;
        dbg! (&note);
        duration_ = i.1;
        vel_ = i.2;
        num_of_channel = i.3;
        // Note On
        track.push(TrackEvent {
            delta: u28::new( dt ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new( note ),
                    vel: u7::new( vel_ ),
                },
            },
        });

        // Note Off (after duration)
        dt += duration_;
        track.push(TrackEvent {
            delta: u28::new( dt ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new( note ),
                    vel: u7::new( vel_),
                },
            },
        });

        dt = 0; // Reset delta time for next note
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
pub fn mk_rnd_midi_dense_n_own_note_duration_4_each_ch (duration: u16, path_to_conf: &String) {
    let duration_u15 = u15::new( duration );
     let mut smf = Smf::new(Header::new(midly::Format::SingleTrack,  Metrical ( duration_u15 ) ) );

    // Create a new track
    let mut track = Track::new();
    let mut uv_note: crate::enums::universum_vox_note =
                      match load_uv_conf( path_to_conf ) {Ok (json ) => json, Err (e) => {eprintln! ("{e}");
                      errMsg0( "Dear user, You need to set json properly.. Look example: {
            \"alg0\":1,\n
            \"num_of_channels\":4,\n
            \"duration\":150,\n
            \"const_duration\":false,\n
            \"deviate_duration\":0,\n
            \"note_duration_on_channel\":[187,284.113],\n
            \"velocity_level\":211,\n
            \"const_velocity\":true,\n
            \"range\":null,\n
            \"bottom\":17,\n
            \"arr\":[63,78]\n-----\nRemark: max range of notes is [0..128]
} "); return;} };
    let mut time = u28::new(0);
    let mut notes: Vec < (u8, u32, u8, u8) > = Vec::new ();
    let mut already_gen_time: u16 = 0;
    let mut num_of_channel: u8 = uv_note.num_of_channels;
    let mut range_of_notes: u8 = 1;
    if uv_note.range.is_none () && uv_note.bottom.is_none () && uv_note.arr.is_none () {
        range_of_notes = 128;
    }
    let mut bottom_note: u8 = 0;
    let mut len_arr = 0usize;
    if uv_note.arr.is_some () { len_arr = uv_note.arr.as_ref().unwrap().len (); } else {
        if let Some ( x ) = uv_note.bottom { bottom_note = x }
        if let Some ( x ) = uv_note.range { range_of_notes = x }
        if bottom_note + range_of_notes > 127 {
            let dt = bottom_note + range_of_notes - 127;
            range_of_notes -= dt;
        }
    }
    let mut note: u8 = 23;
    let mut note_: u8 = 23;
    let mut vel_: u8 = 64;
    let mut duration_: u32 = 67;
    let notes_per_channel = notes.len() / uv_note.num_of_channels as usize;
   let mut count_notes_per_ch = 0usize;
   num_of_channel = 0;
    while already_gen_time < duration {
        if uv_note.const_velocity { vel_ = uv_note.velocity_level } else {
            vel_ = u8__( Some( vel_ ) ) % uv_note.velocity_level;
        }
        if uv_note.const_duration { 
            duration_ = uv_note.duration;
            if uv_note.deviate_duration > 0 { duration_ += u32__( ) % uv_note.deviate_duration; }
         } else {
            duration_ = u32__( ) % uv_note.duration;
        }
        note = u8__( Some( note ) );
        if len_arr > 0 {
            note_ = note %  len_arr as u8;
            note_ = uv_note.arr.as_mut().unwrap() [ note_ as usize ]; 
        } else {
            note_ = note %  range_of_notes;
            note_ = note_ + bottom_note;
        }
        if count_notes_per_ch == notes_per_channel { num_of_channel += 1; count_notes_per_ch = 0;}
        else { count_notes_per_ch += 1; }
        already_gen_time += 1;
        duration_ = match uv_note.note_duration_on_channel.as_ref() {
            Some (x) => x[num_of_channel as usize ],
            _ => uv_note.duration
        };
        notes.push ( (note_, duration_, vel_, num_of_channel ) );
    }
    dbg! (&already_gen_time);
   // errMsg0( "");
   let mut dt = 0u32;
    for i in notes {
        note = i.0;
        dbg! (&note);
        duration_ = i.1;
        vel_ = i.2;
        num_of_channel = i.3;
        // Note On
        track.push(TrackEvent {
            delta: u28::new( dt ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOn {
                    key: u7::new( note ),
                    vel: u7::new( vel_ ),
                },
            },
        });

        // Note Off (after duration)
        dt += duration_;
        track.push(TrackEvent {
            delta: u28::new( dt ),
            kind: TrackEventKind::Midi {
                channel: u4::new( num_of_channel ),
                message: midly::MidiMessage::NoteOff {
                    key: u7::new( note ),
                    vel: u7::new( vel_),
                },
            },
        });

        dt = 0; // Reset delta time for next note
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
pub fn universum_vox (cmd: &String) -> Result < (), Box <dyn Error > >{
    let cmd = cmd.replace ("universum vox", "").trim_end().trim_start ().strn ();
    let (conf_id_, duration_) = crate::split_once( &cmd, " ");
    let mut duration = 15u16;
    let mut conf_id: i64 = 0;
    if let Ok ( x ) = duration_.parse:: <u16> () { duration = x }
    if let Ok ( x ) = conf_id_.parse:: <i64> () { conf_id = x }
    let path_to_conf = crate::get_item_from_front_list( conf_id, true);
    let alg0: u8 = load_uv_conf( &path_to_conf)?.alg0;
    match alg0 {
        0 => { mk_rnd_midi_advanced(duration, &path_to_conf); },
        1 => { mk_rnd_midi_dense(duration, &path_to_conf); },
        2 => { mk_rnd_midi_dense_n_own_note_duration_4_each_ch(duration, &path_to_conf); },
        _ => {}
    }
    Ok ( () )
} 
pub fn load_uv_conf <P: AsRef<Path> >(path: P) -> Result<crate::enums::universum_vox_note, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let uv_note: crate::universum_vox_note = serde_json::from_reader(reader)?; 
    Ok (uv_note )
}
pub fn universum_vox_lst () {
    let dir = crate::cmd_keys::universum_vox_conf( None );
    crate::change_dir (dir, false);
}
//fn
/*
/////////////////// tst variant ////////////////////////////
pub fn mk_rnd_midi (duration: u16) {
    let duration_u15 = u15::new( duration );
     let mut smf = Smf::new(Header::new(midly::Format::SingleTrack,  Metrical ( duration_u15 ) ) );
    let dummy_jsdon = crate::enums::universum_vox_note {
        duration: 15,
        const_duration: false,
        range: None,
        bottom: Some ( 17 ),
        velocity_level: 211,
        const_velocity: true,
        arr: Some (vec! [63,78] ),
    };
    println!("{}", serde_json::to_string (&dummy_jsdon).unwrap () ); 
    let loaded = match load_uv_conf("/tmp/tst_json") {Ok (json ) => json, Err (e) => {eprintln! ("{e}");getkey(); return;} };
    println!("{}", serde_json::to_string (&loaded).unwrap () ); 
    dbg! (&loaded);
    getkey();
    return;
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
/////////////////// tst variant ////////////////////////////
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