#[derive(PartialEq)]
pub(crate) enum cached_data{
    no_rec,
    no_list,
    all_ok
}
#[derive(PartialEq)]
pub(crate) enum cache_state{
    empty,
    ready, 
    no_data_to_add,
    forming
}