run:
	(cd src; rustc -O -C target-cpu=native main.rs; ./main; rm main)

clean: # rm leftover binary if make run is interrupted
	rm src/main

.PHONY: run clean
