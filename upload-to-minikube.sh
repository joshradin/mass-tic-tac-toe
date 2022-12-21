#!/bin/sh

eval $(minikube -p minikube docker-env --shell sh)
docker build -t ghcr.io/joshradin/mass-tic-tac-toe -f ./docker/Dockerfile .
docker build -t jradin37/mass-tic-tac-toe -d nginx
