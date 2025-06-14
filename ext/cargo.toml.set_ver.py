import os
import sys
import re
import codecs
from tam import info_struct, checkArg, get_arg_in_cmd, achtung
import subprocess as sp
import random
class main:
    ver = 1
    rev = 1
    author = info_struct.author
def cpy_file(old: str, new: str):
    cmd = f"cp -af {old} {new}"
    os.system(cmd)
    if not os.path.exists(new):
        print(f"Failed to copy {old} to {new}")
        sys.exit(-5)

def correct_ver_in_toml():
    tomlFile = get_arg_in_cmd("-toml", sys.argv)
    if tomlFile is None:
        print("You didn't set toml file.")
        sys.exit(-7)
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
    crate_name = get_arg_in_cmd("-crate-name", sys.argv)
    if crate_name is None:
        crate_name = ""
    OldVer = re.findall("\d+\.\d+\.\d+", readToml, re.IGNORECASE|re.UNICODE)[0]
    major, minor, patch = OldVer.split(".")
    New_Ver = f"{major}.{minor}{int(patch) + 1}"
    print(OldVer)
    readToml = readToml.replace(OldVer, New_Ver)
    if openToml.write(readToml) == -1:
        print("write to toml been failed")
        sys.exit(-4)
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

def build_pkg():
    build_dir = get_arg_in_cmd("-build-dir", sys.argv)
    if build_dir is None:
        print("You didn't set -build-dir")
        build_dir = str(input("Please, set build directory: "))
    os.system(f"find {build_dir}/dist/ -type f|xargs rm -f")
    achtung(f"dist = {build_dir}/dist/")
    cmd = f"python3 -m build {build_dir}/"
    run_process_w_output(cmd)
def make_all():
    mass_cpy()
    correct_ver_in_toml()
    build_pkg()
make_all()
