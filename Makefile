CARGO=cargo
IMAGE=image.ppm

image:
	@$(CARGO) run > $(IMAGE)
