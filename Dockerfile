# Use a lightweight base image
FROM alpine:latest

# Set the working directory
WORKDIR /app

# Copy the server executable or script to the container
COPY ./server /app/server
COPY ./Rocket.toml /app/Rocket.toml

# Expose port 7890
EXPOSE 7890

# Command to run the server
CMD ["/app/server"]
