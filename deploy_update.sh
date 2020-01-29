#!/bin/sh

# This is used on the server to kill and clean up the currently running container
# Then pull the given tag from docker hub

stop_current () {
    docker stop $1 && docker rm $1
}

stop_current $1
docker pull deciduously0/deciduously-com:$2
docker run -dit -p 3000:8080 deciduously0/deciduously-com