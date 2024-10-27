include .env

build:
	docker build -t tmp-rust-cross-compiler .

compile:
	docker run -v .:/tmp/cross-compile tmp-rust-cross-compiler

sync:
	rsync ./target/release/hw-cross-compile $(USER_NAME)@$(HOST_NAME):~/app/hw-cross-compile

ssh:
	ssh $(HOST_NAME)
