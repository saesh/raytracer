CARGO=cargo
SCENE?=spheres

image:
	@$(CARGO) run --release --example $(SCENE)