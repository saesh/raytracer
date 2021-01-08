SCENE?=spheres

image:
	@cargo run --release --example $(SCENE)

bench:
	@cargo bench

lint:
	@cargo clippy