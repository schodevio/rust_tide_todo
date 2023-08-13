# Using official rust base image
FROM rust:alpine3.17

# Set the application directory
WORKDIR /app

# Install musl-tools to make many crates compile successfully
RUN apk add --no-cache musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

# Expose app port
EXPOSE 1234

# Copy the files to the Docker image
COPY ./ ./
