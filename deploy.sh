#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Helper functions
show_logs() {
    echo -e "${YELLOW}Container logs:${NC}"
    docker-compose logs --tail=50 app
}

show_help() {
    echo -e "${YELLOW}Usage:${NC}"
    echo "  ./deploy.sh deploy    - Deploy the application"
    echo "  ./deploy.sh logs      - Show logs (follow mode)"
    echo "  ./deploy.sh stop      - Stop containers"
    echo "  ./deploy.sh start     - Start containers"
    echo "  ./deploy.sh restart   - Restart containers"
    echo "  ./deploy.sh down      - Stop and remove containers"
    echo "  ./deploy.sh status    - Show container status"
    echo "  ./deploy.sh env       - Create .env file from example"
}

# Check environment variables
check_env() {
    if [ ! -f ".env" ]; then
        echo -e "${RED}Error: .env file not found${NC}"
        echo -e "${YELLOW}Run '${GREEN}./deploy.sh env${YELLOW}' to create it from example${NC}"
        exit 1
    fi
}

# Create .env file from example
create_env() {
    if [ -f ".env" ]; then
        echo -e "${YELLOW}.env file already exists. Do you want to overwrite it? (y/N)${NC}"
        read -r response
        if [[ ! "$response" =~ ^[Yy]$ ]]; then
            exit 0
        fi
    fi
    cp .env.example .env
    echo -e "${GREEN}Created .env file from example${NC}"
    echo -e "${YELLOW}Please edit .env file with your settings${NC}"
}

# Main deployment function
deploy() {
    echo -e "${YELLOW}Starting deployment...${NC}"

    # Check environment
    check_env

    # Check if config file exists
    if [ ! -f "config.yaml" ]; then
        echo -e "${RED}Error: config.yaml not found${NC}"
        exit 1
    fi

    # Build and deploy
    echo -e "${YELLOW}Building and deploying...${NC}"
    docker-compose build --no-cache
    docker-compose up -d

    # Check if container is running
    if [ "$(docker-compose ps -q app)" ]; then
        echo -e "${GREEN}Deployment successful!${NC}"
        show_logs
    else
        echo -e "${RED}Deployment failed!${NC}"
        docker-compose logs app
        exit 1
    fi
}

# Handle command line arguments
case "$1" in
    "deploy"|"")
        deploy
        ;;
    "logs")
        docker-compose logs -f app
        ;;
    "stop")
        docker-compose stop
        ;;
    "start")
        docker-compose start
        ;;
    "restart")
        docker-compose restart
        ;;
    "down")
        docker-compose down
        ;;
    "status")
        docker-compose ps
        ;;
    "env")
        create_env
        ;;
    *)
        show_help
        exit 1
        ;;
esac 