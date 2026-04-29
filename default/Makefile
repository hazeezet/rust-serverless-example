# Makefile for building Rust Lambda functions
# Uses cargo-lambda for cross-compilation to ARM64 Linux

# Variables
RUST_TARGET=aarch64-unknown-linux-gnu
CARGO_OPTS=--release --target $(RUST_TARGET)

# Automatically discover binary names from src/bin directory
BINARIES := $(patsubst src/bin/%.rs,%,$(wildcard src/bin/*.rs))

# Default target
.PHONY: help
help:
	@echo "Rust Lambda Builder"
	@echo "==================="
	@echo "Discovered binaries: $(BINARIES)"
	@echo ""
	@echo "Commands:"
	@echo "  make build         - Build all binaries"
	@echo "  make build-<name>  - Build specific binary"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make check         - Check code"
	@echo "  make test          - Run tests"
	@echo "  make sizes         - Show binary sizes"

# Build all binaries
.PHONY: build
build:
	@echo "Building all binaries..."
	@$(foreach binary,$(BINARIES), $(MAKE) --no-print-directory build-single-$(binary);)
	@$(MAKE) --no-print-directory copy-to-sam
	@echo "All binaries built!"

# Build a specific binary
.PHONY: $(addprefix build-,$(BINARIES))
$(addprefix build-,$(BINARIES)): build-%:
	@$(MAKE) --no-print-directory build-single-$*
	@mkdir -p .aws-sam/build/$*/
	@cp target/lambda/$*/bootstrap .aws-sam/build/$*/
	@echo "Copied $*"

# Internal: cargo lambda build for a single binary
.PHONY: $(addprefix build-single-,$(BINARIES))
$(addprefix build-single-,$(BINARIES)): build-single-%:
	@echo "Building $*..."
	cargo lambda build $(CARGO_OPTS) --bin $*
	@echo "$* built!"

# Copy all built binaries to SAM build directory
.PHONY: copy-to-sam
copy-to-sam:
	@echo "Copying binaries to SAM build directory..."
	@$(foreach binary,$(BINARIES), \
		if [ -f target/lambda/$(binary)/bootstrap ]; then \
			mkdir -p .aws-sam/build/$(binary); \
			cp target/lambda/$(binary)/bootstrap .aws-sam/build/$(binary)/; \
			echo "Copied $(binary)"; \
		fi; \
	)

# Show binary sizes
.PHONY: sizes
sizes:
	@echo "Binary sizes:"
	@$(foreach binary,$(BINARIES), \
		if [ -f target/lambda/$(binary)/bootstrap ]; then \
			ls -lh target/lambda/$(binary)/bootstrap | awk '{print "  $(binary): " $$5}'; \
		else \
			echo "  $(binary): not built"; \
		fi; \
	)

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -rf .aws-sam/build
	@echo "Clean completed!"

# Check code
.PHONY: check
check:
	cargo check
	@echo "Check completed!"

# Run tests
.PHONY: test
test:
	cargo test
	@echo "Tests completed!"

# Format code
.PHONY: fmt
fmt:
	cargo fmt
	@echo "Formatting completed!"
