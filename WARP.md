# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Common Development Commands

### Local Development
```bash
# Run the blog locally (default port 8080)
cargo run

# Run with custom port
PORT=3000 cargo run

# Check and format code
cargo check
cargo fmt
cargo clippy
```

### Content Management
- Blog posts are stored as Markdown files in `content/posts/`
- Create new posts by adding `<slug>.md` files in `content/posts/`
- Posts are accessible at `/posts/<slug>` 
- Banner HTML is in `content/banner.html`

### GCP Deployment Commands
Set required environment variables first:
```bash
export PROJECT_ID={{GCP_PROJECT_ID}}
export GCP_REGION={{GCP_REGION}}
export SERVICE_NAME=gcp-rust-blog
export REPO=blog
```

Build and deploy to Cloud Run:
```bash
# Build and push with Cloud Build
gcloud builds submit --project $PROJECT_ID \
  --tag $GCP_REGION-docker.pkg.dev/$PROJECT_ID/$REPO/$SERVICE_NAME:latest

# Deploy to Cloud Run
gcloud run deploy $SERVICE_NAME \
  --image $GCP_REGION-docker.pkg.dev/$PROJECT_ID/$REPO/$SERVICE_NAME:latest \
  --region $GCP_REGION --platform managed --allow-unauthenticated \
  --port 8080 --ingress all --project $PROJECT_ID
```

### Infrastructure Management
Bootstrap Terraform state (one-time):
```bash
PROJECT_ID={{GCP_PROJECT_ID}} GCS_BUCKET={{YOUR_TF_STATE_BUCKET}} ./scripts/bootstrap-tf-state.sh
```

Apply infrastructure:
```bash
cd infra
terraform init -backend-config="bucket={{YOUR_TF_STATE_BUCKET}}" -backend-config="prefix=gcp-rust-blog/infra"
terraform apply \
  -var="project_id={{GCP_PROJECT_ID}}" \
  -var="project_number={{GCP_PROJECT_NUMBER}}" \
  -var="pool_id={{GCP_WORKLOAD_IDENTITY_POOL}}" \
  -var="provider_id={{GCP_WORKLOAD_IDENTITY_PROVIDER}}" \
  -var="github_owner=dmvianna" \
  -var="github_repo=gcp-rust-blog" \
  -var="cloud_run_url={{CLOUD_RUN_SERVICE_URL}}"
```

## Architecture Overview

### Application Structure
- **Single-file web server**: `src/main.rs` contains the entire Axum-based web application
- **Static content**: Uses Rust's axum framework to serve HTML and render Markdown posts
- **Content-driven**: Blog posts are Markdown files that get converted to HTML at request time
- **Minimal state**: Only loads banner HTML on startup, posts are read from filesystem per request

### Key Components
- **Axum router**: Handles HTTP routing with two main routes:
  - `/` - Homepage with welcome message and post links
  - `/posts/:slug` - Dynamic post rendering from Markdown files
- **Markdown processing**: Uses `pulldown-cmark` for Markdown to HTML conversion
- **Logging**: Configured with `tracing` and `tracing-subscriber` for structured logging

### Dependencies
- `axum 0.7` - Web framework
- `tokio` - Async runtime with full features
- `pulldown-cmark 0.10` - Markdown parser
- `tracing` ecosystem - Logging and observability

### Deployment Architecture  
- **Cloud Run**: Containerized deployment on Google Cloud Platform
- **Load Balancer**: Global HTTP(S) load balancer for custom domain SSL support
- **GitHub Actions CI/CD**: Automated deployment via Workload Identity Federation
- **Artifact Registry**: Container image storage
- **Infrastructure as Code**: Terraform/OpenTofu for WIF setup and IAM roles

### Infrastructure Components
The `infra/` directory contains Terraform configuration for:
- Workload Identity Pool and Provider for GitHub OIDC authentication
- Service account IAM bindings for deployment permissions
- Required project-level roles: Cloud Run admin, Artifact Registry writer, Load Balancer admin
- Global HTTP(S) Load Balancer with SSL certificates for custom domain support
- DNS zone and records for domain management
- Network Endpoint Group (NEG) connecting load balancer to Cloud Run

### Content Structure
```
content/
├── banner.html          # Site header with navigation
└── posts/
    └── first-post.md    # Example blog post
```

## Environment Configuration
- `PORT` - Server port (default: 8080, required for Cloud Run)
- `RUST_LOG` - Log level (default: "info")

## Security Considerations
- Container runs as non-root user
- Uses minimal IAM permissions via dedicated service account
- Secrets managed via environment variables, not baked into images
- Public blog configured with `--allow-unauthenticated`