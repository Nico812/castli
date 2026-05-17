#!/bin/bash

bash ./start_server.sh &
sleep 0.3
bash ./start_client.sh &
