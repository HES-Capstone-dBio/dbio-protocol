# Use alpine unix image
FROM alpine:3.15

# Expose port 8080 for backend server
EXPOSE 8080

# Add the generated binary to the docker image
ADD target/release/dbio-protocol /

# Run the generated binary
CMD ["/dbio-protocol"]