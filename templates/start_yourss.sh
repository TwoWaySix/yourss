#!/bin/bash

# Download youtube-dl
curl -L https://yt-dl.org/downloads/latest/youtube-dl -o youtube-dl
chmod a+rx ./youtube-dl

# User variables
export YOURSS_IP="127.0.0.1"
export YOURSS_FILESERVER_PORT="8880"
export YOURSS_FEEDBUILDER_PORT="8881"
export YOURSS_DOWNLOADER_PORT="8882"
export YOURSS_FRONTEND_PORT="8883"

# Handling all the addresses
export YOURSS_FILESERVER="$YOURSS_IP:$YOURSS_FILESERVER_PORT"
export YOURSS_FEEDBUILDER="$YOURSS_IP:$YOURSS_FEEDBUILDER_PORT"
export YOURSS_DOWNLOADER="$YOURSS_IP:$YOURSS_DOWNLOADER_PORT"
export YOURSS_FRONTEND="$YOURSS_IP:$YOURSS_FRONTEND_PORT"

# Starting programs
./yourss-fileserver & ./yourss-feedbuilder & ./yourss-downloader & ./yourss-frontend &
