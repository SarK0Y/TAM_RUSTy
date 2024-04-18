use crate::{globs18::take_list_adr, errMsg0};
use std::io::BufRead;
pub(crate) fn listing(toFile: String){
    let found_files = take_list_adr("found_files");
    let file = match crate::File::open(&found_files){
            Ok(f) => f,
            _ => return errMsg0("Can't open active list.")
        };
        let reader = crate::BufReader::new(file);
    for (indx, line) in reader.lines().enumerate() {
            let line0 = line.unwrap().as_mut().to_string();
            //
    }
}