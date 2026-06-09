FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

# Install Python 3.12 and system tools useful for kicad-happy
RUN apt-get update && apt-get install -y --no-install-recommends \
    software-properties-common \
    && add-apt-repository -y ppa:deadsnakes/ppa \
    && apt-get update && apt-get install -y --no-install-recommends \
    python3.12 \
    python3.12-venv \
    python3.12-dev \
    python3-pip \
    ngspice \
    poppler-utils \
    git \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Set python3.12 as the default python3
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.12 1

# Install system dependencies for probe-rs USB access
RUN apt-get update && apt-get install -y --no-install-recommends \
    libudev-dev \
    libusb-1.0-0-dev \
    pkg-config \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Clone kicad-happy
RUN git clone https://github.com/aklofas/kicad-happy.git /opt/kicad-happy

# Make scripts executable
RUN find /opt/kicad-happy/skills -name "*.py" -exec chmod +x {} \;
