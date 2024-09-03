pub fn user_home_dir() -> Box (String) {
    static mut home: Lazy <Box <String>> = Lazy::new(|| {Box::new("".strn() )} );
    static mut fst: bool = true;
    if fst {
        *home = "tst".strn();
    }    
    Box("".strn() )
}
