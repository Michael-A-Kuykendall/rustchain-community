#!/bin/bash
# Simple deployment script
export APP_ENV="production"
DEPLOY_DIR="/var/www/app"
readonly CONFIG_FILE="/etc/app.conf"

echo "Starting deployment..."
mkdir -p $DEPLOY_DIR
git pull origin main
npm install --production
npm run build
cp -r dist/* $DEPLOY_DIR/
systemctl restart nginx
echo "Deployment complete!"

# Check if service is running
if systemctl is-active --quiet nginx; then
    echo "Service is running"
else
    echo "Service failed to start"
fi

# Log deployment
echo "$(date): Deployment completed" >> /var/log/deploy.log