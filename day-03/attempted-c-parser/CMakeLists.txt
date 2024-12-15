cmake_minimum_required(VERSION 3.5.0)
project(aoc-2024-03 VERSION 0.1.0 LANGUAGES C)

include_directories(include)

add_executable(aoc-2024-03 
	src/main.c
	src/grammars.c
	include/grammars.h
)