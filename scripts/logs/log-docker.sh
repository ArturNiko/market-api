#!/bin/bash
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <CONTAINER_IMAGE_NAME>"
    exit 1
fi

CONTAINER_IMAGE_NAME=$1
CONTAINER_INDEX=$(docker ps -a | grep "$CONTAINER_IMAGE_NAME" | awk '{print $1}')

if [ -z "$CONTAINER_INDEX" ]; then
    echo "No container found with image name: $CONTAINER_IMAGE_NAME"
    exit 1
fi

# shellcheck disable=SC2046
if [ $(echo "$CONTAINER_INDEX" | wc -w) -gt 1 ]; then
    echo "Multiple containers found with image name: $CONTAINER_IMAGE_NAME"
    echo "Please specify the container index from the following list:"
    echo "$CONTAINER_INDEX"
    exit 1
fi

OUTPUT_FILE="logs/docker_logs.txt"

echo "Capturing logs from container into file: $OUTPUT_FILE"
docker logs "$CONTAINER_INDEX" &> $OUTPUT_FILE
echo "Logs written to $OUTPUT_FILE"