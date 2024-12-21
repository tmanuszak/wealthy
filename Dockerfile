# ===== Builder Stage =====
FROM rust:bookworm as builder

# Set the working dir
WORKDIR /usr/src/app

# Install system dependencies required for SQLx, Postgres, and Node.js
RUN apt-get update && apt-get install -y \
    libpq-dev \
    curl

# Set env variables for nvm and node.js
ENV NVM_DIR /usr/local/nvm
ENV NODE_VERSION 20.18.1

# Install nvm and node.js
RUN mkdir -p $NVM_DIR \
    && curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash \
    && . $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION

# Update path so that nvm and node are available
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# Copy package.json and package-lock.json
COPY package.json package-lock.json ./

# Install npm deps
RUN npm install

# Copy rest of project
COPY . .

# Set environment variables for the build
ENV SQLX_OFFLINE true

# Build for release
RUN cargo build --release

# ===== Runtime Stage =====
FROM debian:bookworm

# Install needed runtime libraries
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/wealthy .

# Copy the static files (e.g., compiled CSS)
COPY --from=builder /usr/src/app/static ./static

# Expose the port your application runs on (e.g., 8080)
EXPOSE ${SERVER_PORT}

# Set the entrypoint
CMD ["./wealthy"]
