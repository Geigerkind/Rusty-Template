FROM archlinux

RUN pacman -Sy \
    && pacman -S --noconfirm git rustup \
    && rustup toolchain install nightly \
    && rustup default nightly
    #&& git clone https://Geigerkind:${GITHUB_TOKEN}@github.com/Geigerkind/Jaylapp \

COPY start.sh /

ENTRYPOINT ["/start.sh"]

CMD ["/bin/bash"]