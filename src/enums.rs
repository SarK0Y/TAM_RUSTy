use serde::{Deserialize, Serialize};
#[derive(PartialEq)]
pub(crate) enum cached_data{
    no_rec,
    no_list,
    all_ok,
    corrupted,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum cache_state{
    empty,
    ready,
    ready0, 
    no_data_to_add,
    forming,
    taken,
    cache_seg_corrupted,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum parse_paths{
    all_files,
    each_name_unique,
    default,
}
#[derive(Debug, Clone, PartialEq)]
pub enum amaze_me{
    ret_indx_n_do_none,
    do_ur_stuff,
    warming(/*times*/ u64),
}
#[derive(Debug, Clone, PartialEq)]
pub enum prompt_modes {
    default,
    glee_uppercases,    
}
#[derive(Debug, Clone, PartialEq)]
pub enum smart_lags {
    well_done ( u128 ),
    too_small_lag ( u128 ),
    failed
}
#[derive(Debug, Clone, PartialEq)]
pub enum calc_kids{
    default_way,
    set_direction,
}
#[derive(Debug, Clone, PartialEq)]
pub enum nvim {
    not_found,
    too_old,
    unknown,
    ok
}
#[derive(Debug, Clone, PartialEq)]
pub enum vim {
    not_found,
    too_old,
    unknown,
    ok
}
use nix::unistd::Pid;  
#[derive(Debug, Clone, PartialEq)]
pub enum threadpool {
    add_new ( Pid ),
    delete ( usize ),
    stop ( usize ),
}
#[derive(Debug, Clone, PartialEq)]
pub enum named_mutex {
    get,
    set,
    unset,
    drop,
    drop_all
}
#[derive(Debug, Clone, PartialEq)]
pub struct custom_mutex {
    pub line_in_register: usize,
    pub owner: *mut u64,
    pub id: u64,
    pub status: bool,
    pub rank: u8
}
#[derive(Debug, Clone, PartialEq)]
pub enum mutex_group < 'a >  {
    set ( &'a String ),
    get ( usize ),
    drop ( usize ),
    find ( *mut u64),
    del_all
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct universum_vox_note {
    pub type_: Option < String >,
    pub alg0: u8,
    pub num_of_channels: u8,
    pub duration: u32,
    pub const_duration: bool,
    pub deviate_duration: u32,
    pub velocity_level: u8,
    pub const_velocity: bool,
    pub note_duration_on_channel: Option < Vec <u32> >,
    pub range: Option <u8 >,
    pub bottom: Option <u8>,
    pub arr: Option < Vec <u8> >
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct universum_vox_wav {
    pub type_: Option < String >,
    pub alg0: u8,
    pub sound_duration: u16, // in seconds
    pub num_of_rnd_samples: u32,
    pub num_of_channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
   pub sample_format: SampleFormat,
   pub bar_sample: f32, 
   pub amplitude: f32,
   pub fading_duration: u32,
    pub fading_step: f32,
    pub silent_step: u64,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SampleFormat { 
    Float,
    Int,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct universum_vox_stub {
    pub type_: Option < String >
}
#[derive(Debug, Clone, PartialEq)]
pub struct  custom_dft {
   pub amplitude: Vec <f32>,
   pub freq: Vec<f32>,
   pub phase: Vec <f32>
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct  freq_range {
   pub from: f32,
   pub to: f32,
   pub step: f32,
   pub overlap: Option <f32>,
   pub frame_len: Option <usize>,
   pub gap_ratio: Option <f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum geom {
    tria {a: f32, b: f32, bar: f32, step: f32, direct: bool }, // (y = a*(x + step * (0..n) ) + b < bar )
    tria_full {a: f32, b: f32, a1: f32, b1: f32, step: f32, bar: f32 }, //a, b, a1, b1, bar/limit (y = ax + b < bar, y1 = ax1 + b1 < bar )
    shark_fin {w: i32, h: f32, lb: Option <f32>}, // width, hight, log base (for rear side)
    half_ellipse {from: f32, to: f32, lb: f32, n: f32}, // *from*, *to*, log base, number of points (1..n)  
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct universum_vox_morph {
    pub type_: Option < String >,
    pub alg0: u8,
    pub num_of_channels: u8,
    pub sample_format: String,
    pub num_of_rnd_samples: Option  <u32 >,
    pub sound_duration: Option  <u32 >,
    pub sample_rate: i32,
    pub step_factor: u32,
    pub fading_duration: u32,
    pub fading_step: f32,
    pub silent_step: u64,
    pub bar_sample: f32,
    pub old_freq: Option <f32 >,
    pub new_freq: Option <f32 >,
    pub step_freq: Option <f32 >,
    pub range: Option <f32 >,
    pub scale: Option <f32 >,
    pub coef: Option <Vec <f32> >,
    pub input_u64: Option <Vec <u64> >,
    pub plus_minus_freq: Option < bool >,
    pub geoms: Option <Vec <geom> >,
    pub select_channel: Option <u8>, 
    pub bandwidth: Option <Vec <freq_range> >, 
    pub dbg_from: Option <u32>,
    pub dbg_to: Option <u32>,
    pub sub_config: Option <String>,
    pub file_in: String,
    pub file_out: String,
}
