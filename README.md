## Installation

### Ubuntu
```
sudo apt update
sudo apt install postgresql-client
```

### MacOS
```
brew install libpq
brew link --force libpq
```


## Running
```
# Run docker
docker-compose up -d

# Run migrations (Only once :D) 
./scripts/migrate.sh
```

