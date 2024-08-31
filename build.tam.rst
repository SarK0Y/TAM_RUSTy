#!/bin/bash
export extra_marker=$(date); uptime; cargo build --release --features=mae; cargo build --features=in_dbg
