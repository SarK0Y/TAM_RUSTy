<div align="center" style="text-align: center;font-size: 30px;">
   <b>GOTO1717.</b>
</div>
The very idea of this sub-project of TAM is, making Rust's macros for practical, educational & researching purposes.<br>
<p style="text-align: center;font-size: 30px;">
   <b>FUNCTIONALITY.</b>
</p>
<ul>
<li> Log variables in function. </li>
<li> <s> Goto. </s></li>
<li> <s> Defer. </s></li>
</ul>
<p style="text-align: center;font-size: 30px;">
   <b>Log variables in function.</b>
</p>
Add deps to Cargo.toml:<br>

```toml
goto1717 = { path = "../ext/goto", version = "0.0.125", optional = true, default-features = false }
rst_lex = { path = "../ext/rst_lex", version = "0.0.68", optional = true }
```

Add features to Your Project: <br>

```toml
tst_macro = ["rst_lex/dev_hell_n_fun", "goto1717/tst"]
macro = ["rst_lex/stable", "goto1717/stable"]
```
Use macro: <br>

```rs
 #[cfg(feature ="tst_macro")]
use goto1717::log_vars;
```

```rs
 #[cfg(feature ="tst_macro")]
 #[log_vars(log_size=3k,log_path=/dev/shm/build_page.log)]
 pub fn Your_Func (.....)
```

# Example w/ TAM:

```bash
mkdir /dev/shm/tst
cd /dev/shm/tst
git clone --branch pre-workable https://github.com/SarK0Y/TAM_RUSTy.git
cd TAM_RUSTy/
cargo build --no-default-features --features in_dbg --features=mae --features=tst_macro --features=tam >  /tmp/mess 2>&1
```
https://github.com/SarK0Y/TAM_RUSTy/blob/52d12558ddb78f213811721561585a477f485de7/main/src/basic.pg.rs#L121 <br>
 # If You don't want to log variable, just add prefix 'nolog_' to its name:

 ```rs
let mut nolog_myvar: usize = 0;
```
 <b> Links: </b> <br>
 <b>Rolling guide of TAM (Topnotch Practical ways to use Console/Terminal):<b> https://alg0z8n8its9lovely6tricks.blogspot.com/2024/08/tam-guide-of-features-smart-tricks.html <br>
 <b>DISCORD:</b> https://discord.gg/X9RBbtCN (Alg0Z). <br>
 <b>TELEGRAM:</b> https://t.me/+N_TdOq7Ui2ZiOTM6 (Alg0Z). <br>
 <b>ALG0Z RU:</b> https://dzen.ru/alg0z <br>
 <b>ALG0Z EN:</b> https://alg0z.blogspot.com <br>
 <b>ChangeLog:</b> https://alg0z8n8its9lovely6tricks.blogspot.com/2023/09/tam-changelog.html <br>
 <b>FORUM:</b> https://www.neowin.net/forum/topic/1430114-tam/ <br>
 <b>E-MAIL:</b> sark0y@protonmail.com <br>
 <b>GITHUB:</b> https://github.com/SarK0Y/TAM_RUSTy.git <br>
 <b>YouTube:</b> https://www.youtube.com/@evgeneyknyazhev968 <br>
 <b>Twitter_X:</b> https://x.com/SarK0Y8 <br>
 Donations: https://boosty.to/alg0z/donate https://zap-hosting.com/en/shop/donation/1f0c83845d810df04ca74e56238399f7/ <br>
 # <p align=center> <b> MAKE CONSOLE GREAT AGAIN.🤘 </b> </p>
 
# my the Best Wishes to You 🙃
