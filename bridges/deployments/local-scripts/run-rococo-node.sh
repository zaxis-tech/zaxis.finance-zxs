#!/bin/bash

# Run a development instance of the Rococo Substrate bridge node.
# To override the default port just export ROCOCO_PORT=9955
#
# Note: This script will not work out of the box with the bridges
# repo since it relies on a Z-Axis binary.

ROCOCO_PORT="${ROCOCO_PORT:-9955}"

RUST_LOG=runtime=trace,runtime::bridge=trace \
./target/debug/zaxis --chain=rococo-dev --alice --tmp \
    --rpc-cors=all --unsafe-rpc-external --unsafe-ws-external \
    --port 33044 --rpc-port 9934 --ws-port $ROCOCO_PORT \
