#!/bin/sh

clang main.c -c -o c-land.lib 
mv c-land.lib ../rust-land/c-land.lib