#! /usr/bin/python3

import os
import subprocess
import sys


BASE_DIR = os.path.abspath(".netbox_docker")

def run_command(command, cwd=None):
    try:
        print(f"Running: {' '.join(command)}")
        subprocess.run(command, cwd=cwd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error: {e}")
        sys.exit(1)


def create_docker_compose(version):
    compose_content = f"""
version: '3.4'

services:
  netbox:
    image: netboxcommunity/netbox:{version}
    ports:
      - "8000:8080"
    env_file:
      - env/netbox.env
    depends_on:
      - postgres
      - redis
      - redis-cache

  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: netbox
      POSTGRES_USER: netbox
      POSTGRES_PASSWORD: netbox
    volumes:
      - netbox-postgres-data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine

  redis-cache:
    image: redis:7-alpine

volumes:
  netbox-postgres-data:
  """

    os.makedirs(BASE_DIR, exist_ok=True)
    with open(os.path.join(BASE_DIR, "docker-compose.yml"), "w") as f:
        f.write(compose_content.strip())


def create_env_file(superuser_name: str, superuser_email: str, superuser_password: str):
    env_dir = os.path.join(BASE_DIR, "env")
    os.makedirs(env_dir, exist_ok=True)
    env_content = f"""
SUPERUSER_NAME={superuser_name}
SUPERUSER_EMAIL={superuser_email}
SUPERUSER_PASSWORD={superuser_password}

DB_NAME=netbox
DB_USER=netbox
DB_PASSWORD=netbox
DB_HOST=postgres

REDIS_HOST=redis
REDIS_PORT=6379
REDIS_DB=0

REDIS_CACHE_HOST=redis-cache
REDIS_CACHE_PORT=6379
REDIS_CACHE_DB=1

ALLOWED_HOSTS=*
DB_WAIT_TIMEOUT=30
"""
    with open(os.path.join(env_dir, "netbox.env"), "w") as f:
        f.write(env_content.strip())


def setup_netbox(version: str, superuser_name: str, superuser_email: str, superuser_password: str):
    print("Setting up NetBox Docker...")
    create_docker_compose(version)
    create_env_file(superuser_name, superuser_email, superuser_password)
    print("Pulling Docker images...")
    run_command(["docker-compose", "pull"], cwd=BASE_DIR)
    print("Starting NetBox conainers...")
    run_command(["docker-compose", "up", "-d"], cwd=BASE_DIR)
    print("Success: NetBox is running at http://localhost:8000")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Setup NetBox using Docker")
    parser.add_argument("--version", type=str, default="latest", help="NetBox Docker image tag (e.g, v4.0, latest)")
    parser.add_argument("--superuser-name", type=str, default="admin", help="Username of the superuser (default: admin)")
    parser.add_argument("--superuser-email", type=str, default="admin@example.com", help="Email of the superuser")
    parser.add_argument("--superuser-password", type=str, default="admin", help="Password of the superuser (default: admin)")

    args = parser.parse_args()
    setup_netbox(
            version=args.version,
            superuser_name=args.superuser_name,
            superuser_email=args.superuser_email,
            superuser_password=args.superuser_password
        )
