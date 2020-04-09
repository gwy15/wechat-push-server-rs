#!/bin/bash
./build/server-rs > tests/server.log &
SERVER_PID=$!
sleep 1 # wait for the server to warm up
pytest tests -v && kill $SERVER_PID || kill $SERVER_PID
