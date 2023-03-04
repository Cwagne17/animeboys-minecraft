#!/bin/bash
DATE=$(date '+%Y-%m-%d')

aws s3 cp --recursive /minecraft/mc-server s3://animeboys-backup/backup/$DATE/mc-server
aws s3 cp --recursive /minecraft/forge s3://animeboys-backup/backup/$DATE/forge