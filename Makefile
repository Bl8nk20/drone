all: prepare

dependency: 
	cd build && cmake .. --graphviz=graph.dot && dot -Tpng graph.dot -o graphImage.png
	
install:
	sudo apt-get install gcc g++ cmake make doxygen git llvm pkg-config curl zip unzip tar python3-dev clang-format clang-tidy

prepare:
	rm -rf build
	mkdir build

compile:
	cd build && cmake -S .. -B . 

testing: compile 
	cd build/tests && ./unit_tests

conan_d:
	rm -rf build
	mkdir build
	cd build && conan install .. -s build_type=Debug --output-folder=. --build missing -s compiler.cppstd=17

conan_r:
	rm -rf build
	mkdir build
	cd build && conan install .. -s build_type=Release --output-folder=. --build missing -s compiler.cppstd=17
