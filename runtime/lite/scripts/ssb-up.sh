#!/bin/bash

# Function to find a container by name or partially matching name


copy-files() {
  find_container() {
  local container_name="ssb-pubs"
  local running_containers=$(docker ps --format '{{.Names}}')
  for container in $running_containers; do
    if [[ "$container" == "$container_name" || "$container" =~ ^"$container_name".* ]]; then
      echo "$container"
      return 0
    fi
  done
  echo "Container not found."
  return 1
}


# Find the container
container=$(find_container "ssb-pubs")

# Check if container was found
if [[ $? -eq 0 ]]; then
  # Execute the command inside the container
  echo "$container"
  docker exec -it "$container" bash -c "cp /home/node/config /home/node/.ssb/"
else
  echo "Exiting..."
fi
}

# Start the Network
start_service() {
  docker-compose --project-name ssb up -d  
}

# Stop the Network
stop_service() {
  docker-compose --project-name ssb down  
}

# Initilize the file, creating the folder for mounting the config for pub
initialize() {
  mkdir test
  

cat > ./test/config <<EOF
{
  "connections": {
    "incoming": {
      "net": [
        {
          "scope": "public",
          "host": "0.0.0.0",
          "transform": "shs",
          "port": 8008
        }
      ]
    },
    "outgoing": {
      "net": [
        {
          "transform": "shs"
        }
      ]
    }
  }
}
EOF
}

case "$1" in
  start)
    start_service 
    ;;
  stop)
    stop_service
    ;;
  copy)
    copy-files 
    ;;
  init)
    initialize
    ;;
  *)
    echo "Invalid command. Please enter init, start, copy or stop."
    ;;
esac