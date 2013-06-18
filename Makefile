all: 
		rustc --lib math.rc -L .
		rustc tests.rs -L .

clean: 
		rm -rf *o math