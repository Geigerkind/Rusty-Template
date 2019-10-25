FROM archlinux

COPY /home/runner/work/Jaylapp/Jaylapp/Webclient /

RUN pacman -Sy \
    && pacman -S --noconfirm git npm chromium \
    && cd Webclient \
    && npm install @angular/cli \
    && npm install \
    && ng test