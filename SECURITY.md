# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability, please report it privately by:

1. **Email**: Contact the repository owner directly
2. **GitHub Security**: Use GitHub's private vulnerability reporting
3. **Issues**: For non-sensitive security improvements, create a public issue

## What NOT to Include in Public Commits

⚠️ **Never commit these sensitive files:**

- `infra/prod.tfvars` - Contains project IDs and configuration
- `infra/*.auto.tfvars` - Auto-loaded variable files
- `.env` or `.envrc` - Environment variables
- `*.json` - Service account key files
- Any file containing API keys, passwords, or credentials

## Safe to Include in Public Repository

✅ **These files are safe to make public:**

- All `.tf` files - Infrastructure as Code definitions
- Application source code (`src/`)
- Documentation (`docs/`, `README.md`)
- Configuration templates (`*.template` files)
- `Dockerfile` and build scripts
- GitHub Actions workflows (they reference secrets, don't contain them)

## For Contributors

When contributing:

1. **Use the template files** - Copy `.template` files and fill in your values
2. **Check your commits** - Ensure no sensitive data is included
3. **Use GitHub Secrets** - Store sensitive values in repository secrets
4. **Follow least privilege** - Request minimal permissions needed

## Production Security

The deployed application follows security best practices:

- **Workload Identity Federation** - No service account keys stored
- **Least privilege IAM** - Minimal permissions for each service account
- **HTTPS only** - All traffic encrypted in transit
- **Container security** - Non-root execution, minimal attack surface
- **Organization policies** - Additional security controls at GCP org level

For detailed security architecture, see [docs/SECURITY.md](docs/SECURITY.md).