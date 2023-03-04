locals {
  bucket_name = "animeboys-minecraft-backup"
}

# S3 Bucket

resource "aws_s3_bucket" "minecraft-backup" {
    bucket = local.bucket_name
}

resource "aws_s3_bucket_acl" "minecraft-backup" {
    bucket = aws_s3_bucket.minecraft-backup.id
    acl = "private"
}

# Upload Files

resource "aws_s3_object" "server" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "mc-server/minecraft_server.1.19.3.jar"
  source = "files/minecraft_server.1.19.3.jar"
}

resource "aws_s3_object" "forge" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key = "mc-server/forge-1.16.5-36.2.2-installer.jar"
  source = "files/forge-1.19.3-44.1.0-installer.jar"
}

resource "aws_s3_object" "setup" {
    bucket = aws_s3_bucket.minecraft-backup.id
    key    = "setup/minecraft.service"
    source = "files/minecraft.service"
}