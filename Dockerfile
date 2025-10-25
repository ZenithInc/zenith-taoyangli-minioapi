# Use base image with pre-installed composer dependencies
FROM registry.zks.testing.icu/taoyangli/pc-base:latest

# Copy application code
COPY ./ /data/project

WORKDIR /data/project

# Regenerate optimized autoloader with actual code
RUN composer dump-autoload --no-dev -o

ENTRYPOINT ["php", "/data/project/bin/hyperf.php", "start"]

