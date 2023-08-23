# Nothing to see here

This is an internal experiment for Team Plattform.
Feel free to browse, but you will not learn anything useful here. :boom:

## Highlights

* Per app, per env least privilege (run cloud build with `application` or with a new `ci` SA)
* No longer needs `./helm/$repo/charts` and the bundled `common-$verison.tgz`
* Easily handle `kubernetes` `firebase` `cloud function` `cloud run` deployment types
* Clean arch for "no cluster cron" with Cloud Scheduler -> Cloud Build


## Tweaks needed

Some work needs to be done to play with Cloud Build as a first class citizen.
Below is a list of things that needed doing as part of a demo for Entur.

Add SLACK_URL to SM with terraform?

Resource Manager API

### Preconditions

- [ ] Existing repo in AR
- [ ] Able to access `helm` cloud builder (see below)
  - gcloud auth configure-docker europe-west1-docker.pkg.dev
- [ ] Access to TF statefile

### Build a cloud builder for helm

- [ ] Build https://github.com/GoogleCloudPlatform/cloud-builders-community/tree/master/helm
  - [ ] CI/CD for this image (renovate)

```sh
CLOUDSDK_COMPUTE_REGION=europe-west1
CLOUDSDK_COMPUTE_ZONE=europe-west1-b
CLOUDSDK_CONTAINER_CLUSTER=kub-ent-sbx-001
GCLOUD_PROJECT=ent-kub-sbx
```
### IAM changes

- [ ] roles/logging.logWriter for `application`
- [ ] artifactregistry.repositories.uploadArtifacts
  - [ ] Cloud Build Service Account
- [ ] Kubernetes Engine Developer (cleanup)
- [ ] Storage Admin in ent-tfapps-shr-001 (cleanup)
- [ ] Owner
- [ ] Compute Network Viewer in ent-networks-shr-sbx-something (cleanup)

### App factory changes

- [ ] Connect git to cloud build
  - [ ] Restrict to target repo
