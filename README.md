#Getting started
```
sudo apt-get install 
```

Install nightly

```
# install rustup
curl https://sh.rustup.rs -sSf | sh

rustup install nightly

# start postgresql and seed the database
sudo -u postgres psql -f init.sql
cargo install diesel_cli --no-default-features --features "postgres"
diesel migration run

cargo run
```

# Routes

- ```/``` index and data visualization
- ```/api/data``` dataset
- ```/api/kmeans``` cluster centroids according to kmeans
