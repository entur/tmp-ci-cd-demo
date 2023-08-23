# Terraform Entur init configuration
module "init" {
  source      = "github.com/entur/terraform-google-init//modules/init?ref=v0.3.0"
  app_id      = var.appfactory_id
  environment = var.environment
}

# Create a GCS bucket using the Entur cloud-storage module
module "cloud-storage" {
  source                      = "github.com/entur/terraform-google-cloud-storage//modules/bucket?ref=v0.2.0"
  init                        = module.init
  name_override               = "rocketlaunch"
  disable_offsite_backup      = true  # not needed for this example
  force_destroy               = true  # nothing to keep in the bucket
  versioning                  = false # no need for versioning
  generation                  = 1     # generation #number 001
  create_kubernetes_resources = true
  # lifecycle_rules_override = [
  #   {
  #     action = {
  #       type = "Delete"
  #     }
  #     condition = {
  #       age = 2
  #       with_state = "ARCHIVED"
  #     }
  #   }
  # ]
  # storage_purpose = "standard"
}

# create a temporary file to upload to the storage bucket
resource "local_file" "rocket_file" {
  content  = "We have liftoff!"
  filename = "rocket.txt"
}

# Upload a text file as an object to the storage bucket that the apps can read.
resource "google_storage_bucket_object" "rocket_file_object" {
  name         = "rocket.txt"
  source       = "rocket.txt"
  content_type = "text/plain"
  bucket       = module.cloud-storage.cloud_storage_bucket.id
  #lifecycle rule ignore md5 hash
  lifecycle {
    ignore_changes = [
      md5hash,
      detect_md5hash
    ]
  }
}

# create secret:
resource "google_secret_manager_secret" "rocket_secret_001" {
  secret_id = "ROCKET_SECRET"
  labels = {
    rocket = "getting-started"
  }
  replication {
    user_managed {
      replicas {
        location = "europe-west1"
      }
      replicas {
        location = "europe-west4"
      }
    }
  }
  project = module.init.app.project_id
}

# create random password
resource "random_password" "password" {
  keepers = {
    # Generate new password when we switch this number
    secret_id = 1
  }

  length           = 16
  special          = true
  override_special = "!#$%&*()-_=+[]{}<>:?"
}

# add secret version
resource "google_secret_manager_secret_version" "rocket_secret_version_001" {
  secret      = google_secret_manager_secret.rocket_secret_001.id
  secret_data = random_password.password.result
}
