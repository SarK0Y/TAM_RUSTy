use std::collections::HashMap;
pub(crate) type cache_t = HashMap<String, HashMap<usize, Vec<String>>>;
pub(crate) type entry_cache_t = HashMap<usize, Vec<String>>;
pub(crate) type patch_t = HashMap<String, Vec<String>>;