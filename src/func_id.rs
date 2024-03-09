pub const initSession_: i64 = 1;
pub const find_files: i64 = 2;
pub const  check_substr: i64 = 3;
pub const  mk_cmd_file_: i64 = 4;
pub const  run_cmd: i64 = 5;
pub const  build_page: i64 = 6;
pub const  init_page_struct_: i64 = 7;
pub const  page_struct_: i64 = 8;
pub const form_cmd_line_default: i64 = 9;
pub const  exec_cmd: i64 = 10;
pub const  hotKeys: i64 = 11;
pub const  main_update: i64 = 12;
pub const read_midway_data_: i64 = 13;
pub const INS_: i64 = 14;
pub const viewer_: i64 = 15;
pub const run_cmd_viewer_: i64 = 16;
pub const where_is_last_pg_: i64 = 17;
pub const calc_num_files_up2_cur_pg_: i64 = 18;
pub const background_fixing_: i64 = 19;
pub fn get_func_name(func_id: i64) -> &'static str {
    //let max = i64::MAX;
    let ret = match func_id {
        initSession_ => "initSession",
        find_files => "find_files",
        check_substr => "check_substr",
        mk_cmd_file_ => "mk_cmd_file",
        run_cmd => "run_cmd",
        build_page => "build_page",
        init_page_struct_ => "init_page_struct",
        page_struct_ => "page_struct",
        form_cmd_line_default => "form_cmd_line_default",
        exec_cmd => "exec_cmd",
        hotKeys => "hotKeys",
        main_update => "main_update",
        read_midway_data_ => "read_midway_data",
        INS_ => "INS",
        viewer_ => "viewer",
        run_cmd_viewer_ => "run_cmd_viewer",
        where_is_last_pg_ => "where_is_last_pg",
        calc_num_files_up2_cur_pg_ => "calc_num_files_up2_cur_pg",
        background_fixing_=> "background_fixing_",
        _ => "unknown func",
    };
    return ret;
}
