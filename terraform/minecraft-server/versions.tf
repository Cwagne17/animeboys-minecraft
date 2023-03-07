terraform {
  required_version = ">= 1.2.9"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 4.36"
    }
  }

  backend "s3" {
    bucket = "animeboys-minecraft"
    key    = "live/mc-server/terraform.tfstate"
    region = "us-east-1"

    dynamodb_table = "animeboys-minecraft"
    encrypt        = true

    profile = "cwagne17-personal"
  }
}

provider "aws" {
  profile = "cwagne17-personal"
  region  = "us-east-1"
}