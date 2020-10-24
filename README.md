# Geth exporter for Prometheus in Rust

## Metrics page example

```
# HELP bitcoind_blockchain_sync Blockchain sync info
# TYPE bitcoind_blockchain_sync gauge
bitcoind_blockchain_sync{type="current"} 8937608
bitcoind_blockchain_sync{type="highest"} 8937608
bitcoind_blockchain_sync{type="progress"} 1
# HELP geth_gas_price Current gas price in wei
# TYPE geth_gas_price gauge
geth_gas_price 1000000000
# HELP geth_latest Latest block information
# TYPE geth_latest gauge
geth_latest{hash="0x9b708aa0a99aca70f9af0ebf1fd6aff37a96323da0976b18d0d6de0742987756"} 8937608
# HELP geth_mempool_size Mempool information
# TYPE geth_mempool_size gauge
geth_mempool_size{type="size"} 92
# HELP geth_network Client network
# TYPE geth_network gauge
geth_network{value="ropsten"} 1
# HELP geth_peers Connected peers
# TYPE geth_peers gauge
geth_peers{version="all"} 50
# HELP geth_version Client version
# TYPE geth_version gauge
geth_version{value="Geth/v1.9.23-stable-8c2f2715/linux-amd64/go1.15.3"} 1
```

## Environment Variable
| | |
| -- | -- |
| GETH_EXPORTER_LISTEN | port to listen |
| GETH_EXPORTER_NODE | URL of the Geth server connecting to |
| GETH_EXPORTER_INTERVAL | interval to update metrics |

## Usage
### local environment
```
export GETH_EXPORTER_LISTEN=0.0.0.0:8000
export GETH_EXPORTER_NODE=http://geth:8545/
export GETH_EXPORTER_INTERVAL=3000
cargo run
```

### Docker

```
docker run \
  -p 8000:8000 \
  -e GETH_EXPORTER_LISTEN=0.0.0.0:8000 \
  -e GETH_EXPORTER_NODE=http://geth:8545/ \
  -e GETH_EXPORTER_INTERVAL=3000 \
  geth-exporter-rust
```
