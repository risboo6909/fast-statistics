build:
	cargo build --release

clean:
	rm -rf target

test:
	cargo test

deploy:
	docker build .
