use std::collections::{HashMap, HashSet};
pub(crate) type cache_t = HashMap<String, HashMap<usize, Vec<String>>>;
pub(crate) type entry_cache_t = HashMap<usize, Vec<String>>;
pub(crate) type patch_t = HashMap<String, String>;
pub(crate) type no_esc_t = HashSet<String>;