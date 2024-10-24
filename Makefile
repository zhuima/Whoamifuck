# 项目名称
PROJECT_NAME := whoamifuck
AUTHOR := zhuima
VERSION := 0.1.0
DESCRIPTION := Whoamifuck，zhuima first open source tool. This is a tool written by rust to detect intruders, after the function update, is not limited to checking users' login information.

# 目标平台
PLATFORMS := linux darwin windows
ARCHITECTURES := amd64 arm64

# Rust 工具链
CARGO := cargo
RUSTC := rustc

# 输出目录
TARGET_DIR := target
RELEASE_DIR := $(TARGET_DIR)/release

# 默认目标
.PHONY: all
all: fmt check clippy build

# 格式化代码
.PHONY: fmt
fmt:
	@echo "Formatting code..."
	@$(CARGO) fmt

# 检查代码
.PHONY: check
check:
	@echo "Checking code..."
	@$(CARGO) check

# 运行 Clippy
.PHONY: clippy
clippy:
	@echo "Running Clippy..."
	@$(CARGO) clippy -- -D warnings

# 构建项目
.PHONY: build
build: fmt check clippy
	@echo "Building project..."
	@$(CARGO) build --release

# 清理构建产物
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	@$(CARGO) clean

# 多平台构建
.PHONY: release
release: fmt check clippy
	@echo "Building release versions for multiple platforms..."
	@mkdir -p $(RELEASE_DIR)
	@for platform in $(PLATFORMS); do \
		for arch in $(ARCHITECTURES); do \
			target="$${platform}-$${arch}"; \
			echo "Building for $${target}"; \
			case $$platform in \
				windows) \
					extension=".exe" ;; \
				*) \
					extension="" ;; \
			esac; \
			case $$platform in \
				linux) \
					rust_target="$${arch}-unknown-linux-gnu" ;; \
				darwin) \
					rust_target="$${arch}-apple-darwin" ;; \
				windows) \
					rust_target="$${arch}-pc-windows-msvc" ;; \
			esac; \
			RUSTFLAGS="-C target-feature=+crt-static" $(CARGO) build --release --target $${rust_target}; \
			cp $(TARGET_DIR)/$${rust_target}/release/$(PROJECT_NAME)$${extension} $(RELEASE_DIR)/$(PROJECT_NAME)-$${target}$${extension}; \
		done; \
	done

# 运行测试
.PHONY: test
test:
	@echo "Running tests..."
	@$(CARGO) test

# 显示帮助信息
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  all      - Format, check, run Clippy, and build the project (default)"
	@echo "  fmt      - Format the code"
	@echo "  check    - Check the code"
	@echo "  clippy   - Run Clippy"
	@echo "  build    - Build the project"
	@echo "  clean    - Clean build artifacts"
	@echo "  release  - Build release versions for multiple platforms"
	@echo "  test     - Run tests"
	@echo "  help     - Show this help message"
