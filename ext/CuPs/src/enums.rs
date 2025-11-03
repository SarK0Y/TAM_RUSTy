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
#[derive(Debug, Clone, PartialEq)]
pub enum smart_lags {
    well_done ( u128 ),
    too_small_lag ( u128 ),
    failed
} 
