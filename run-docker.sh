docker run \
  --interactive \
  --tty \
  --rm \
  --mount type=bind,source=./,target=/app \
  --entrypoint bash \
  cleanmail:latest
