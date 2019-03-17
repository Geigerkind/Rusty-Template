# Getting the package DB up to date
pacman -Sy --noconfirm;

# Utility stuff
pacman -S --noconfirm htop unzip zip pkgconfig guetzli git;

# Compiler
pacman -S --noconfirm make clang rustup python-pip nodejs npm;

# Database related
pacman -S --noconfirm mariadb unixodbc;

# Webserver related
pacman -S --noconfirm nginx nginx-mod-brotli;

# NPM packages
npm install -g html-minifier;
npm install -g less;
npm install -g csso-cli;
npm install -g uglify-es;
npm install -g typescript;

# PIP packages
pip install webp-converter;