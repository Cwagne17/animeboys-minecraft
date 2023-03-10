locals {
  bucket_name = "animeboys-minecraft-backup"
}

# S3 Bucket

resource "aws_s3_bucket" "minecraft-backup" {
  bucket = local.bucket_name
}

resource "aws_s3_bucket_acl" "minecraft-backup" {
  bucket = aws_s3_bucket.minecraft-backup.id
  acl    = "private"
}

# Upload Files

resource "aws_s3_object" "server" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "mc-server/minecraft_server.1.19.3.jar"
  source = "files/installers/minecraft_server.1.19.3.jar"
}

resource "aws_s3_object" "setup" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "setup/minecraft.service"
  source = "files/minecraft.service"
}

# Modded Minecraft Files

resource "aws_s3_object" "forge" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/forge-1.18.2-40.2.0-installer.jar"
  source = "files/installers/forge-1.18.2-40.2.0-installer.jar"
}

resource "aws_s3_object" "mantle" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/mods/mantle-1.18.2-1.9.27.jar"
  source = "files/mods/Mantle-1.18.2-1.9.27.jar"
}

resource "aws_s3_object" "tconstruct" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/mods/tconstruct-1.18.2-3.5.2.40.jar"
  source = "files/mods/TConstruct-1.18.2-3.5.2.40.jar"
}

resource "aws_s3_object" "jei" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/mods/jei-1.18.2-forge-10.2.1.1002.jar"
  source = "files/mods/jei-1.18.2-forge-10.2.1.1002.jar"
}

resource "aws_s3_object" "mowziesmobs" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/mods/mowziesmobs-1.18.2-1.5.32.jar"
  source = "files/mods/mowziesmobs-1.5.32.jar"
}

resource "aws_s3_object" "geckolib" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/mods/geckolib-forge-1.18-3.0.57.jar"
  source = "files/mods/geckolib-forge-1.18-3.0.57.jar"
}

resource "aws_s3_object" "twilightforest" {
  bucket = aws_s3_bucket.minecraft-backup.id
  key    = "forge/mods/twilightforest-1.18.2-4.1.1423-universal.jar"
  source = "files/mods/twilightforest-1.18.2-4.1.1423-universal.jar"
}