CARGO=cargo
CONVERT=convert
SCENE?=spheres
IMAGE_NAME=$(SCENE)
TARGET_DIR=out

build:
	@$(CARGO) build

release:
	@$(CARGO) build --release

test:
	@$(CARGO) test

image:
	@$(CARGO) run --release --example $(SCENE) > $(TARGET_DIR)/$(IMAGE_NAME).ppm
	@$(CONVERT) $(TARGET_DIR)/$(IMAGE_NAME).ppm $(TARGET_DIR)/$(IMAGE_NAME).png
	@rm $(TARGET_DIR)/$(IMAGE_NAME).ppm