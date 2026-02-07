# Build both frontends
build:
    #!/usr/bin/env bash
    set -e

    echo "Building both frontends..."

    # Clean previous builds
    rm -rf "{{justfile_directory()}}/dist"
    mkdir -p "{{justfile_directory()}}/dist"

    # Build main frontend
    echo "Building main frontend..."
    cd "{{justfile_directory()}}/frontend"
    trunk build --release
    mv dist/* "{{justfile_directory()}}/dist/"

    # Build assembly frontend
    echo "Building assembly frontend..."
    cd "{{justfile_directory()}}/frontend-assembly"
    trunk build --release
    mkdir -p "{{justfile_directory()}}/dist/assembly"
    mv dist/* "{{justfile_directory()}}/dist/assembly/"

    echo "Build complete! Output in dist/"

# Run tests
test:
    cargo test

# Deploy vrbkapages pod
deploy:
    #!/usr/bin/env bash
    set -e

    echo "Deploying vrbkapages pod..."

    # Stop existing pod if running
    if podman pod exists vrbkapages-pod; then
        echo "Stopping existing pod..."
        podman pod stop vrbkapages-pod
        podman pod rm vrbkapages-pod
    fi

    # Create and start new pod
    echo "Creating new pod..."
    podman play kube vrbkapages-pod.yaml

    echo "Deployment complete!"
    echo "Pod status:"
    podman pod ps
