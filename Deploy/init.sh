DOMAIN='yajla.com'
HOST_USER='root'
HOST_IP='51.38.114.9'
DB_PASSWORD=$(cat /root/Keys/db_password)

function initCertificates {
  pacman -S --noconfirm certbot python certbot-dns-ovh ca-certificates
  if [ ! -f "/etc/ssl/certs/ca-certificates.crt"]; then
    cd /etc/ssl/certs
    cat *.pem >> ca-certificates.crt
    cd ~
  fi
  # See: https://certbot-dns-ovh.readthedocs.io/en/stable/
  chmod -R 600 ~/Keys/ovh.ini
  # Requires user input
  certbot certonly --dns-ovh --dns-ovh-credentials ~/Keys/ovh.ini -d ${DOMAIN} -d smtp.${DOMAIN}
  echo "0 0,12 * * * root python -c 'import random; import time; time.sleep(random.random() * 3600)' && certbot renew" | tee -a /etc/crontab > /dev/null
}

function installZopfli {
  git clone https://github.com/google/zopfli
  cd zopfli
  make zopflipng
  cp zopflipng /usr/bin/
  cd ..
  rm -rf zopfli
}

function initNginx {
  pacman -S --noconfirm nginx nginx-mod-brotli
  cp ~/Jaylapp/Deploy/conf/nginx.conf /etc/nginx/
  systemctl enable nginx
  systemctl start nginx
}

function initMariaDb {
  pacman -S --noconfirm mariadb
  mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql
  cp ~/Jaylapp/Deploy/conf/my.conf /etc/
  systemctl enable mysqld
  systemctl start mysqld
  mysql -u root mysql -e "ALTER USER 'root'@'localhost' IDENTIFIED BY '${DB_PASSWORD}'"
  systemctl restart mysqld
  cd ~/Jaylapp/Database
  bash merger.sh
  mysql -uroot -p${DB_PASSWORD} < merge.sql
  rm merge.sql
  cd ~
  systemctl restart mysqld
}

function initPostfix {
  pacman -S --noconfirm postfix
  cp ~/Jaylapp/Deploy/conf/virtual /etc/postfix/
  cp ~/Jaylapp/Deploy/conf/main.cf /etc/postfix/
  postmap /etc/postfix/virtual
  systemctl enable postfix
  systemctl start postfix
}

function initSSH {
  for filename in /Jaylapp/Deploy/ssh/*.pub; do
    if [ ! -f "${filename}" ]; then
      continue
    fi
    cat ${filename} > ~/.ssh/authorized_keys
  done
}

function installRust {
  pacman -S --noconfirm rustup
  rustup toolchain install nightly
  rustup default nightly
}

function initServer {
  # Requires user input
  passwd root
  useradd -m yajla

  pacman -Sy
  pacman -S --noconfirm git npm guetzli libwebp htop clang
  installRust
  installZopfli
  npm install -g html-minifier
  # Requires user input
  npm i -g @angular/cli
  # See: https://git-scm.com/book/de/v2/Git-Tools-Credential-Storage
  git config --global credential.helper
  cp ~/Keys/.git-credentials ~/
  git clone https://github.com/Geigerkind/Jaylapp
  cd /root/Jaylapp/Webclient
  # Requires user input
  npm install
  cd /root
  cp /root/Jaylapp/Deploy/conf/backend.service /etc/systemd/system/

  initSSH
  initCertificates
  initNginx
  initMariaDb
  initPostfix
}

initServer