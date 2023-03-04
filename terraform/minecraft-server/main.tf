locals {
    name = "animeboys-minecraft-server"

    user_data = <<-EOT
    #!/usr/bin/env bash
    sudo yum -y install java
    sudo yum -y install setools
    /* sudo systemctl start polkit */

    # minecraft
    sudo mkdir /minecraft
    sudo mkdir /minecraft/mc-server
    cd /minecraft
    # for initial configuration enable copy from S3 cold storage to empty EBS:
    aws s3 cp --recursive s3://<s3-bucket-name>/mc-server/ mc-server/
    sudo chown -R ec2-user:ec2-user /minecraft

    # systemd service
    sudo aws s3 cp s3://<s3-bucket-name>/setup/minecraft.service /etc/systemd/system
    sudo systemctl daemon-reload
    sudo systemctl enable minecraft
    sudo systemctl start minecraft
    EOT
}

# VPC

module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
  version = "3.19.0"

  name = local.name
  cidr = "10.0.0.0/16"

  azs = ["us-east-1a", "us-east-1b"]
  public_subnets = ["10.0.1.0/24", "10.0.2.0/24"]

  enable_dns_hostnames = true
  enable_dns_support   = true
}

module "security_group" {
  source  = "terraform-aws-modules/security-group/aws"
  version = "~> 4.0"

  name        = local.name
  description = "Security group for animeboys usage with EC2 instance"
  vpc_id      = module.vpc.vpc_id

  # Allow all traffic
  ingress_cidr_blocks = ["0.0.0.0/0"]
  ingress_rules       = ["all-all"]
  egress_rules        = ["all-all"]
}

# SSH Key

resource "aws_key_pair" "ssh" {
    key_name = local.name
    public_key = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCadx7gQ4w3T/uoL9gnZDyof8i8bwK/IWVC/JMYkATd9b5jzFxxBX+upWdFXuMIRuThwxBFyOZijNsAmoMPMnRDJybt6shwyQHDiDPmxMhPKTrZKWaay2Bcas7nTWpD7X/UYothlGOlGHafW43DFU8XL3jOGIw9x3rxAZyCkWW8e12nfIuUbnLgUYw5OgYTMiVdi9mvo8Kq56hwInRggTqCVfkoTgsQPk7taukkl6UDJYEbwCxi0lwn1/wpV0XqAeAkitTif9Xwg45SaiSd7/7p9jp0JH6yd/0a48S/J3v88y0oyZc7Sj9vOcqAzvwlL1NBFTsPApQ1oqv7lWWmVHy88fm6G9SIwIgVdhiXNyhni2BYbgKvJeF253h16oRFC4TT8xQZarGp/gmWxuiuGZ6OvjaGqDSWPNnn1QbUdDhoU1kVoVjRadCs5rpXZsnYKFEPT/kYpRn+yQSeSffn1yKsiFZR7HcyXD8BKL6JU8EQx4tUrM1H7+a3Ugdzzj48QqcigZax/gU/TJi++0On7rZYXJjFcjevCtXmwJNL+uMN3YwCy5ViZKysj04+TAhXkY6D1Bhvrob3ORTD4dslVg3JpFmjj+U3lLWMSuQT2L7wBsbF7MmAmGOJVuWNvEI/4Dk/P/1hn87NQtW7gNtK+lfY42DDL0zXW1z9Gk2iZNG2Pw== cwagne17@animeboys.com"
}

# EC2 Instance

module "ec2" {
    source = "terraform-aws-modules/ec2-instance/aws"
    version = "4.3.0"
    
    name = local.name

    # Amazon Linux 2 Kernel 5.10 AMI 2.0.20230221.0 x86_64 HVM gp2
    ami = "ami-006dcf34c09e50022"
    instance_type = "t2.medium"

    # Network Configuration
    subnet_id = element(module.vpc.public_subnets, 0)
    vpc_security_group_ids = [module.security_group.security_group_id]   
    associate_public_ip_address = true
    disable_api_stop = false
    
    # IAM Configuration
    create_iam_instance_profile = true
    iam_role_description        = "IAM role for EC2 instance"
    iam_role_policies = {
        AdministratorAccess = "arn:aws:iam::aws:policy/AdministratorAccess"
    }

    # Server Configuration
    /* user_data_base64            = base64encode(local.user_data) # we need to define this
    user_data_replace_on_change = true */
    key_name = aws_key_pair.ssh.key_name
    
    # EBS Configuration
    /* ebs_block_device = [{
            device_name = "/dev/sda1"
            volume_size = 8
            volume_type = "gp2"
            delete_on_termination = false
    }] */
}