run:
	(cd src; rustc main.rs; ./main; rm main)

clean: # rm leftover binary if make run is interrupted
	rm src/main

.PHONY: run clean
