# 使用更新的 Rust 版本
FROM rust:1.74-slim as builder

# 安装构建依赖
RUN apt-get update && \
    apt-get install -y \
    git \
    pkg-config \
    libssl-dev \
    musl-tools \
    make \
    perl \
    gcc \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/whoamifuck

# 初始化 git 仓库
COPY .git ./.git
COPY . .

# 确保 git 仓库正确初始化
RUN git config --global --add safe.directory /usr/src/whoamifuck

# 设置构建环境变量
ENV RUST_BACKTRACE=1

# 构建项目
RUN cargo build --release

# 使用更小的基础镜像
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y \
    procps \
    ca-certificates \
    pkg-config \
    libssl-dev \
    bash-completion \
    && rm -rf /var/lib/apt/lists/*

# 复制构建产物
COPY --from=builder /usr/src/whoamifuck/target/release/whoamifuck /usr/local/bin/whoamifuck


# 设置入口点
ENTRYPOINT ["whoamifuck"]