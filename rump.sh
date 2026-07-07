#!/bin/bash
HOST_CFLAGS="-Wno-error=implicit-function-declaration -fcommon" ./buildrump.sh -V NOGCCERROR=1 -o ./rump_output tools
HOST_CFLAGS="-Wno-error=implicit-function-declaration -fcommon" ./buildrump.sh -V NOGCCERROR=1 -o ./rump_output build
cp ./rump_output/lib/librump/librump.a
