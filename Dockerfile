# Use alpine unix image
FROM alpine:3.15

# Add the generated binary to the docker image
ADD target/release/dbio-protocol .

# Run the generated binary
CMD ["./dbio-protocol"]