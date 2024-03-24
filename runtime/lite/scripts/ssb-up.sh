#!/bin/bash

# Function to find a container by name or partially matching name
find_container() {
  local container_name=$1
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

copy_files() {
# Get the container name or pattern to search for
# read -p "Enter container name (or part of the name): " container_name

# Find the container
container=$(find_container "ssb-pubs")

# Check if container was found
if [[ $? -eq 0 ]]; then
  # Get the command to execute
#   read -p "Enter the command to execute: " command

  # Execute the command inside the container
  echo "$container"
  docker exec -it "$container" bash -c "cp /home/node/config /home/node/.ssb/"
else
  echo "Exiting..."
fi
}

start_service() {
  docker-compose --project-name ssb up -d  
}

stop_service() {
  docker-compose --project-name ssb down  
}

initialize() {
  mkdir ssb-test
  

cat > ./ssb-test/config <<EOF
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

create_invite(){
    container=$(find_container "ssb-pubs")
    invite=$(docker exec -it "$container" bash -c "ssb-server invite.create 10")
    echo $invite

    # Get the IP address of the Docker container
    container_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $container)

    # Edit the invite with container ip
    internal_invite=$(echo "$invite" | sed 's/0.0.0.0/'$container_ip/g)
    echo $internal_invite
}

accept_invite(){
    invite=$1
    consumer_container=$(find_container "ssb-consumer")
    producer_container=$(find_container "ssb-producer")

    consumer_accept=$(docker exec -it "$consumer_container" bash -c "ssb-server invite.accept $invite")
    sleep 3
    producer_accept=$(docker exec -it "$producer_container" bash -c "ssb-server invite.accept $invite")
    echo $consumer_accept
    echo $producer_accept
}

copy_secrets_to(){
    consumer_container=$(find_container "ssb-consumer")
    producer_container=$(find_container "ssb-producer")
    pubs_container=$(find_container "ssb-pubs")
    
    mkdir -p ./secret

    docker cp  $consumer_container:/root/.ssb/secret ./secret/producer_secret
    docker cp  $producer_container:/root/.ssb/secret ./secret/consumer_secret
    docker cp  $pubs_container:/home/node/.ssb/secret ./secret/pubs_secret
}
start_specific_service() {
  service=$1
  docker-compose --project-name ssb up -d $service
}

pub_whoami(){
    container=$(find_container "ssb-pubs")
    whoami=$(docker exec -it "$container" bash -c "ssb-server whoami")
    echo $whoami
}


case "$1" in
  start)
    initialize
    start_service 
    ;;
  stop)
    stop_service
    rm -rf ssb-test
    rm -rf secret
    ;;
  copy)
    copy_files 
    ;;
  create-invite)
    create_invite
    ;;
  accept-invite)
    accept_invite $2
    ;;
  start-service)
    start_specific_service $2
    ;;
  copy-secret)
    copy_secrets_to
    ;;
  pub-whoami)
    pub_whoami
    ;;
  *)
    echo "Invalid command. Please enter start, copy or stop."
    ;;
esac