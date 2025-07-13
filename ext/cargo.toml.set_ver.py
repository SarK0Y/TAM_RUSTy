import os
import sys
import re
import codecs
import copy
from sark0y_tam._tam import checkArg, get_arg_in_cmd, get_arg_in_cmd_from, achtung
import subprocess as sp
import random
def cpy_file(old: str, new: str):
    cmd = f"cp -af {old} {new}"
    os.system(cmd)
    if not os.path.exists(new):
        print(f"Failed to copy {old} to {new}")
        sys.exit(-5)
def get_ver_from_strn (strn: str, crate_name: str) -> str:
    pat_str = f"{crate_name}\s*=\s*\{{.*\}}"	
    print (f"pattern {pat_str}")
    dep = re.findall(pat_str,
     strn, re.IGNORECASE|re.UNICODE)
    if dep == []:
    	print (f"fn get_ver_from_strn provides empty list")
    	sys.exit(-12)
    else: dep = dep[0]
    ver = re.findall("version\s*=\s*\"\d+\.\d+\.\d+\"",
     dep, re.IGNORECASE|re.UNICODE)[0]
    if ver == []: 
    	print (f"fn get_ver_from_strn can't get ver")
    	sys.exit(-13)
    print (f"fn get_ver_from_strn ver = {ver}")
    return ver
def correct_ver_in_all_toml():
	tomlFile, from_indx = correct_ver_in_toml(0)
	print (f"0while tomlFile, indx = {tomlFile},\n {from_indx}")
	while tomlFile is not None:
		print (f"while tomlFile, indx = {tomlFile}, {from_indx}")
		tomlFile, from_indx = correct_ver_in_toml( from_indx +1 )
def correct_ver_in_toml(from0: int) -> (str|None, int):
    fn_name = "correct_ver_in_toml"
    tomlFile, indx = get_arg_in_cmd_from(from0, "-toml", sys.argv)
    if tomlFile is None:
        print(f"You didn't set toml file {{ searched from {from0} }}.")
        return tomlFile, indx
    if not os.path.exists(tomlFile):
        print(f"{tomlFile} ain't existed")
        sys.exit(-8)
    bkp_toml = f"cp -af {tomlFile} {tomlFile}_bkp"
    os.system(bkp_toml)
    if not os.path.exists(f"{tomlFile}_bkp"):
        print(f"Failed to backup {tomlFile}")
        sys.exit(-3)
    ver = re.compile('\d+', re.UNICODE)
    try:
        openToml = open(tomlFile, "r+")
    except FileExistsError:
        print(f"toml = {os.path.exists(tomlFile)}")
        sys.exit(-1)
    if openToml is None or openToml == -1: sys.exit(-2)
    readToml = f"{openToml.read()}"
    openToml.flush()
    openToml.seek(0)
    print(f"{readToml =}")
    """"""
    crate_name, _ = get_arg_in_cmd_from(indx, "-crate-name", sys.argv)
    Line = re.findall("version\s*=\s*\"\d+\.\d+\.\d+\"",
     readToml, re.IGNORECASE|re.UNICODE)[0] if crate_name is None else get_ver_from_strn (readToml, crate_name)
    OldVer = re.findall("version\s*=\s*\"\d+\.\d+\.\d+\"", Line, re.IGNORECASE|re.UNICODE)[0]
    major, minor, patch = OldVer.split(".")
    patch = patch.replace('"', "")
    New_Ver = f"{major}.{minor}.{int(patch) + 1}\""
    print(f"OldVer = {OldVer}, new one: {New_Ver}")
    old_Line = copy.deepcopy(Line)
    print(f"Line {Line}")
    Line = Line.replace(OldVer, New_Ver)
    readToml = readToml.replace(old_Line, Line)
    print (f"{readToml=}\n{tomlFile=}")
    if openToml.write(readToml) == -1:
        print("write to toml been failed")
        sys.exit(-4)
    print (f"Exit {fn_name}")
    return tomlFile, indx
def mass_cpy():
    SRC = []
    for s in sys.argv:
        if s[:4] == "-src":
            SRC.append(s.removeprefix("-src"))
    src_set = set(SRC)
    if len(src_set) != len(SRC):
        print("List of files to copy has duplicates.")
        sys.exit(-6)
    for i in SRC:
        src0 = get_arg_in_cmd(f"-src{i}", sys.argv)
        dst0 = get_arg_in_cmd(f"-dst{i}", sys.argv)
        if dst0 is None:
            print(f"You didn't set -dst{i}")
            continue
        if src0 is None:
            print(f"You didn't set -src{i}")
            continue
        cpy_file(src0, dst0)
def run_process_w_output(cmd):
    stop_code = "∇\n"
    cmd = [f"{str(cmd)};echo '\n{stop_code}'", ]
    stderr0_name = f"/tmp/build_tam_err{str(random.random())}"
    stderr0 = open(stderr0_name, "w+")
    stdout0_name = f"/tmp/build_tam_out{str(random.random())}"
    stdout0 = open(stdout0_name, "w+")
    p = sp.Popen(cmd, shell=True, stderr=stderr0, stdout=stdout0)
    read_stdout0 = open(stdout0_name)
    read_stdout0.flush()
    read_stdout0.seek(0)
    for line in iter(read_stdout0.readline, b''):
        if line == stop_code:
            break
        if line !="":
            print(line)
    print(stderr0.read())

def make_all():
    mass_cpy()
    correct_ver_in_all_toml()
if "__main__" == __name__:
	make_all()
