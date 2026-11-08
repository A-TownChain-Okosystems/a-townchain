# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Docker-Compose Integration Test (K7.8)
Verifies that all 7 services are defined and have health checks.
Agent: Aurora #2 (6a275618)
"""
import pytest
import os
import yaml


@pytest.mark.integration
class TestDockerComposeIntegration:
    """Test docker-compose.yml configuration for 7 services."""

    @pytest.fixture
    def compose_file(self):
        path = os.path.join(os.path.dirname(__file__), '..', '..', 'docker', 'docker-compose.yml')
        if not os.path.exists(path):
            pytest.skip("docker-compose.yml not found")
        with open(path) as f:
            return yaml.safe_load(f)

    def test_compose_has_7_services(self, compose_file):
        """All 7 services are defined."""
        services = compose_file.get('services', {})
        expected = {'core', 'blockchain', 'frontend', 'gateway', 'contracts', 'franchise', 'game'}
        actual = set(services.keys())
        missing = expected - actual
        assert not missing, f"Missing services: {missing}"
        assert len(actual) == 7

    def test_all_services_have_health_checks(self, compose_file):
        """Every service has a healthcheck."""
        for name, svc in compose_file.get('services', {}).items():
            assert 'healthcheck' in svc, f"Service '{name}' missing healthcheck"

    def test_all_services_have_restart_policy(self, compose_file):
        """Every service has restart: unless-stopped."""
        for name, svc in compose_file.get('services', {}).items():
            assert svc.get('restart') == 'unless-stopped', f"Service '{name}' missing restart policy"

    def test_network_defined(self, compose_file):
        """Custom network is defined."""
        assert 'networks' in compose_file
        assert 'atc-net' in compose_file['networks']

    def test_volume_defined(self, compose_file):
        """Blockchain data volume exists."""
        assert 'volumes' in compose_file
        assert 'blockchain-data' in compose_file['volumes']

    def test_service_ports(self, compose_file):
        """Key services expose correct ports."""
        services = compose_file.get('services', {})
        assert '4000:4000' in str(services.get('core', {}).get('ports', []))
        assert '3000:3000' in str(services.get('frontend', {}).get('ports', []))
        assert '80:80' in str(services.get('gateway', {}).get('ports', []))

    def test_service_dependencies(self, compose_file):
        """Services depend on core being healthy."""
        services = compose_file.get('services', {})
        for name in ['blockchain', 'contracts', 'franchise', 'game']:
            deps = services.get(name, {}).get('depends_on', {})
            assert 'core' in deps, f"Service '{name}' should depend on core"

    def test_dockerfiles_exist(self):
        """All Dockerfiles referenced in compose exist."""
        docker_dir = os.path.join(os.path.dirname(__file__), '..', '..', 'docker')
        for df in ['Dockerfile.core', 'Dockerfile.frontend', 'Dockerfile.gateway']:
            path = os.path.join(docker_dir, df)
            assert os.path.exists(path), f"Missing {df}"
