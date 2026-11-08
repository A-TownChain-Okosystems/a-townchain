# ARCHITECTURE.md — atc-deploy
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── configs/
│   └── network_config.atc
├── docker/
│   ├── dockerfile_builder.atc
│   └── image_manager.atc
├── k8s/
├── kubernetes/
│   ├── helm_chart.atc
│   └── k8s_deployer.atc
├── requirements.txt
├── scripts/
│   ├── deploy_script.atc
│   └── health_check.atc
├── src/
│   ├── __init__.py
│   ├── deploy/
│   │   └── __init__.py
│   ├── node_setup/
│   │   └── __init__.py
│   ├── rollback/
│   │   └── __init__.py
│   └── scripts/
│       └── __init__.py
└── terraform/
    ├── dns_manager.atc
    └── infra_manager.atc
```

## Module Descriptions
- **src/**: Infrastructure deployment CLI and automated deployment controller package.
- **docker/**: Dockerfiles and image build management scripts for ATC services.
- **k8s/** / **kubernetes/**: Kubernetes manifests and Helm charts for deploying clusters to staging and production.
- **terraform/**: Infrastructure-as-Code (IaC) modules provisioning cloud compute instances, storage buckets, and VPC networks.
- **requirements.txt**: Python package requirements for cloud API and IaC driver scripts.

## Build System
Python 3.10+ packaging, HashiCorp Terraform CLI, Kubernetes `kubectl` and Helm 3.

## Dependencies
Python 3.10+, `python-terraform`, `kubernetes-client`, `pyyaml`, `jinja2`, `boto3`.
