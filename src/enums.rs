#[derive(PartialEq)]
pub(crate) enum cached_data{
    no_rec,
    no_list,
    all_ok
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