FROM ontuotu-docker.pkg.coding.net/taoyangli/base/hyperf/hyperf:8.0-alpine-v3.15-swoole

LABEL maintainer="douya" version="1.0"

RUN apk add tzdata && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
&& echo "Asia/Shanghai" > /etc/timezone \
&& apk del tzdata

# update
RUN set -ex \
    && apk update \
    && cd /etc/php8 \
    # - config PHP
    && { \
        echo "upload_max_filesize=128M"; \
        echo "post_max_size=128M"; \
        echo "memory_limit=1024M"; \
        echo "date.timezone=Asia/Shanghai"; \
    } | tee conf.d/php8-overrides.ini \
    # ---------- clear works ----------
    && rm -rf /var/cache/apk/* /tmp/* /usr/share/man \
    && echo -e "\033[42;37m Build Completed :).\033[0m\n"

COPY ./ /data/project

WORKDIR /data/project

RUN cd /data/project \
    && composer install --no-dev -o

ENTRYPOINT ["php", "/data/project/bin/hyperf.php", "start"]

