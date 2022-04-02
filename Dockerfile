# Use Rust 1.59
FROM rust:1.59.0-alpine

# Expose port 8080 for backend server
EXPOSE 8080

# Add the generated binary to the docker image
ADD target/release/dbio-protocol /

# Run the generated binary
CMD ["dbio-protocol"]