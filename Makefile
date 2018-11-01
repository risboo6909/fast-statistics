build:
	cargo build --release

clean:
	rm -rf target

test:
	cargo test

interactive:
	docker build -t fast_stat .
	docker run -it -t fast_stat

wheels:
	docker run --rm -v `pwd`:/io quay.io/pypa/manylinux1_x86_64 /io/build-wheels.sh

