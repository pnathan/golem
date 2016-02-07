install:
	cd golem && cargo build --release
	cp ./golem/target/release/golem ~/bin
