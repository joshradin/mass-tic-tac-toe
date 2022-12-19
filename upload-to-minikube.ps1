& minikube -p minikube docker-env --shell powershell | Invoke-Expression
echo $env:DOCKER_HOST
docker build -t ghcr.io/joshradin/mass-tic-taco -f .\docker\Dockerfile .\ --platform linux