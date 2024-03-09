
# Project TAM.
# GOAL:

The very reason of this project is to make Your life easy at least for a little bit thanks to efficient automation of daily routine.

# FUNCTIONS:

For now, TAM makes possible to find files in whatever directory and run them with chosen viewers. hmmm.. yea, looks rather boring ain't it??? :)) well, let's look on some examples & details will show You how this "boring" feature can make Your life better.

# Examples:

So, actually we have hella number of files & we (in many cases) need to use regex to make searching through that endless damn heap efficient, but the most of us have no time to learn such stuff + regex software have some differences. 2nd moment, we need not just run files, but run them with different options. So, let's deal w/ example..

 python3 ./tam.py -path0 "/tst"  -find_files -tmp_file "/tmp/tst02" -in_name ".mp4" -view_w "vlc --sout-x264-b-bias=-15" -view_w "smplayer"  -cols 2 -rows 15 -col_w 100 -in_name "some"
 
 -path0 sets folder to search stuff.<br>
 -find_files activates function to search.<br>
 -tmp_file sets tmp files (actually, the're two tmp files: in our case, /tmp/norm_tst02 & /tmp/err_tst02).<br>
 -in_name sets keyword.<br>
 -view_w sets viewer w/ options.<br>
 -cols sets number of columns.<br>
 -rows sets number of rows.<br>
 -col_w sets width of column.<br>
+++++++++++++<br>
 This command forms table of found files, each file gets a number + we see list of viewers (each viewer has own key number too)..<br>
 
 To run file, we write "\<key number of viewer\> \<key number of file\>", then press Enter. for instance, "0 2" runs file (key number "2") w/ viewer (key number "0").<br>
 
 Command "np" shows next page/table.<br>
 "pp" - previous page.<br>
 "go2 \<number of page/table\>"<br>
 "0p" - 1st page.<br>
 "lp" - last one.<br>
 "fp <key number of file>" shows full path to chosen file.<br>
 ctrl + c to exit.<br>
 # Short guide how to use TAM.
 [![Watch the video](https://img.youtube.com/vi/szqe--xIs8I/maxresdefault.jpg)](https://youtu.be/szqe--xIs8I)
 # How to switch TAM's Konsoles.
 [![Watch the video](https://img.youtube.com/vi/dkRJKsMTyoc/maxresdefault.jpg)](https://youtu.be/dkRJKsMTyoc)
 # Supported Platforms:
 
 So far, TAM has been tested only for Linux. However, theoretically this variant must work on FreeBSD, NetBSD & MacOS quite smoothly (but i don't guarantee it).
# my the Best Wishes to You 🙃
