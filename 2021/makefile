#File Name
F?=three.rs

#Templates for when generating file
CPPTEMPLATE?=~/Desktop/CP/templates/simple.cpp
RSTEMPLATE?=~/Desktop/CP/templates/simple.rs

#compiler flags and stuff
CFLAGS=-Wextra -Wall -pedantic -O3 -std=c++14
CC=g++
PYTHON=python3
RFLAGS=-O
RUSTC=rustc

#Specify a language with make lang where lang is the language (curretnly "cpp" or "py")
#language defaults to cpp
all:
	$(CC) $(CFLAGS) $(F)
	./a.out

py:
	$(PYTHON) $(F)

rs:
	$(RUSTC) $(RFLAGS) $(F) -o a
	./a

gencpp:
	cp $(CPPTEMPLATE) ./$(F)

genrs:
	cp $(RSTEMPLATE) ./$(F)

.PHONY: all py gencpp rs
