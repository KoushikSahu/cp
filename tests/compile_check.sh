#!/bin/bash

set -e

for file in `find . -not -path '*/\.*' -type f \( ! -iname ".*" \)`
do
	echo "Running test for $file"
	if [[ $file == *.py ]]
	then
		python3 -m py_compile $file
	elif [[ $file == *.cpp ]]
	then
		g++ -std=c++2a $file
	elif [[ $file == *.c ]]
	then
		gcc $file
	elif [[ $file == *.md ]]
	then
		echo "Ignoring compile test for markdown files"
	elif [[ $file == *Makefile ]]
	then
		echo "Ignoring compile test for makefile"
	elif [[ $file == *.sh ]]
	then
		echo "Ignoring compile test for scripts"
	elif [[ $file == *.rs ]]
	then
		rustc -Awarnings $file
	elif [[ $file == *.skiptests ]]
	then
		echo "Ignoring compile test for files marked to skip tests"
	elif [[ $file == *.toml ]]
	then
		echo "Ignoring compile test for toml files"
	elif [[ $file == *.lock ]]
	then
		echo "Ignoring compile test for lock files"
	else
		echo "Unknown file type"
		exit 1
	fi            
done

