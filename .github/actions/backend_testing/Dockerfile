FROM archlinux

RUN pacman -Sy \
    && pacman -S --noconfirm git rustup \
    && echo ${GITHUB_TOKEN} \
    && git clone https://Geigerkind:${GITHUB_TOKEN}@github.com/Geigerkind/Jaylapp \
    && rustup toolchain install nightly \
    && rustup default nightly