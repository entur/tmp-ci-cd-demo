terraform {
  backend "gcs" {
    # getstarted = id, replace this with your project appfactory id defined in variables.tf
    bucket = "ent-gcs-tfa-getstarted"
  }
}
