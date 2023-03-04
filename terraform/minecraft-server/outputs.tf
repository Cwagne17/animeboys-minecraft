output "arn" {
    value = module.ec2.arn
    description = "The ARN of the instance"
}

output "iam_instance_profile_arn" {
    value = module.ec2.iam_instance_profile_arn
    description = "The ARN of the instance profile"
}

output "instance_state" {
    value = module.ec2.instance_state
    description = "The state of the instance"
}

output "public_ip" {
    value = module.ec2.public_ip
    description = "The public IP address of the instance"
}