all: 
		rustc --lib math.rc
		rustc tests.rs -L .

clean: 
		rm -rf *o math