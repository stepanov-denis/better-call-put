# Better Call Put
Application for trading on the MOEX via the T-Invest API.
## Prerequisites
* Copy `config.example.yaml` in `config.yaml` and add your tokens:
```bash
cp config.example.yaml config.yaml
```
## Run app in docker locally
* Run app in the background
```bash
docker-compose up -d
```
* View logs:
```bash
docker-compose logs -f
```
* Stop and remove containers:
```bash
docker-compose down
```
* Stop and remove containers including volumes:
```bash
docker-compose down --volumes
```