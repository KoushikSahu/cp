#!/bin/bash

for file in `find . -name "*.cpp" -type f`
do
	echo "compiling file $file"
	g++ -std=c++17 $file
done

