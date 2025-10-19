terraform {
  backend "gcs" {
    # Configure with -backend-config at init time, e.g.:
    #   -backend-config="bucket=gs://YOUR_TF_STATE_BUCKET"
    #   -backend-config="prefix=your-project-name/infra"
  }
}
