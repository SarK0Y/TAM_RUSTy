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
pub enum nvim {
    not_found,
    too_old,
    unknown,
    ok
}

