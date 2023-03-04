#!/usr/bin/env bash
# Install Java
wget https://download.oracle.com/java/19/latest/jdk-19_linux-x64_bin.rpm
sudo rpm -Uvh jdk-19_linux-x64_bin.rpm
sudo yum -y install setools

# minecraft
sudo mkdir /minecraft
sudo mkdir /minecraft/mc-server
cd /minecraft
sudo chown -R ec2-user:ec2-user /minecraft
aws s3 cp --recursive s3://animeboys-minecraft-backup/mc-server/ mc-server/

# systemd service
sudo aws s3 cp s3://animeboys-minecraft-backup/setup/minecraft.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl enable minecraft
sudo systemctl start minecraft