pub(crate) fn fix_screen_count(num: usize){
    std::thread::spawn(move||{
        for i in 0..num{
            let (prnt, subcmd) = split_once(&read_prnt(), ":>:");
            if subcmd == "no_upd_scrn"{
                set_prnt(&prnt, -164547841);
                return;
            }
            clear_screen();
            let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
            let mut data = "".to_string();
            let num_pg = crate::get_num_page(-5555555121);
            let num_pgs = crate::where_is_last_pg();
            crate::swtch::print_viewers();
            crate::swtch::print_pg_info();
            if num_pg < num_pgs || num_pgs ==0 {crate::pg18::build_page(&mut ps);}
            println!("{}", crate::get_prnt(-1));
            crate::pg18::form_cmd_newline_default();
           std::thread::sleep(std::time::Duration::from_millis(615));        
           let (prnt, subcmd) = split_once(&read_prnt(), ":>:");
            if subcmd == "no_upd_scrn"{
                set_prnt(&prnt, -164547841);
                return;
            }
        }
    });
}