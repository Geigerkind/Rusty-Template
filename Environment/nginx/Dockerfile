FROM archlinux/base

COPY ./mirrorlist /etc/pacman.d/

RUN pacman -Sy && \
    pacman -S --noconfirm nginx nginx-mod-brotli && \
    mkdir -p /etc/nginx/cert

COPY nginx.conf /etc/nginx/
copy dhparam.pem /etc/nginx/cert/
copy jaylapp.crt /etc/nginx/cert/
copy jaylapp.key.pem /etc/nginx/cert/

ENTRYPOINT ["/usr/bin/nginx"]