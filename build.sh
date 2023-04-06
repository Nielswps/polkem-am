#!/bin/bash

podman build -t docker.io/nielswps/account-manager:latest . && \
podman push nielswps/account-manager