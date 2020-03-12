.PHONY: dev

dev:
	docker run -it --rm --name rust -v $(shell pwd):/workspace -w /workspace -u nobody rust bash

cc:
	rustc $(t).rs --out-dir ./out
