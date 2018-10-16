build:
	cargo build --release

clean:
	rm -rf target

test:
	cargo test

docker:
	docker build -t fast_stat .

docker_run: docker
	docker run -it fast_stat
