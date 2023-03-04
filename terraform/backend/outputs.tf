output "bucket_name" {
  description = "Name of S3 bucket"
  value       = aws_s3_bucket.backend.bucket
}

output "bucket_region" {
  description = "AWS region S3 bucket is in"
  value       = aws_s3_bucket.backend.region
}