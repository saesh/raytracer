CARGO=cargo
CONVERT=convert
IMAGE_NAME=image

image:
	@$(CARGO) run > $(IMAGE_NAME).ppm
	@$(CONVERT) $(IMAGE_NAME).ppm $(IMAGE_NAME).png
	@rm $(IMAGE_NAME).ppm