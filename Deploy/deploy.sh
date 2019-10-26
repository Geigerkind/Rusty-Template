# TODO: Setup automatic deploy from github

HOST_USER='root'
HOST_IP='51.38.114.9'
NUM_CORES=$(nproc)
DB_PASSWORD=$(cat /root/Keys/db_password)

function cleanAssetCache {
  cd /root/cache/assets/
  for filename in *.png *.jpg *.jpeg; do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    if [[ ! -f "/root/Jaylapp/Webclient/src/assets/${filename}" ]]; then
      rm ${filename}
      rm ${filename%.*}.webp &> /dev/null # Ignore error if it had been deleted already
    fi
    DIR=$(dirname "${filename}")
    if [ -z "$(ls -A ${DIR})" ]; then
      rm -rf ${DIR}
    fi
  done
  cd /root/
}
function optimizeJpg {
  cd /root/Jaylapp/Webclient/src/assets/
  MEDIA_DIR='/root/cache/assets/'
  for filename in *.jpg *.jpeg; do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done

    BASEFILENAME=$(basename "${filename}");
    PATHTOFILE=$(dirname "${filename}");
    TARGETDIR="${MEDIA_DIR}${PATHTOFILE}";

    if [ ! -d "${TARGETDIR}" ]; then
        mkdir -p ${TARGETDIR};
    fi

    guetzli --quality 84 --nomemlimit ${filename} ${TARGETDIR}/${BASEFILENAME} > /dev/null 2> /dev/null &
  done
  cd /root
}
function optimizePng {
  cd /root/Jaylapp/Webclient/src/assets/
  MEDIA_DIR='/root/cache/assets/'
  for filename in *.png; do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done

    BASEFILENAME=$(basename "${filename}");
    PATHTOFILE=$(dirname "${filename}");
    TARGETDIR="${MEDIA_DIR}${PATHTOFILE}";

    if [ ! -d "${TARGETDIR}" ]; then
        mkdir -p ${TARGETDIR};
    fi

    zopflipng --iterations=5 --filters=2me --lossy_8bit --lossy_transparent -y ${filename} ${TARGETDIR}/${BASEFILENAME} > /dev/null 2> /dev/null &
  done
  cd /root
}
function convertToWebp {
  for filename in /root/cache/assets/*.png /root/cache/assets/*.jpg /root/cache/assets/*.jpeg; do
    if [ ! -f "${filename}" ]; then
      continue
    fi

    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done
    webpc -q=30 ${filename} ${filename%.*}.webp > /dev/null 2> /dev/null &
  done
}
function optimizeAssets {
  echo "Optimizing assets"
  mkdir -p /root/cache/assets &> /dev/null
  cleanAssetCache
  optimizeJpg
  optimizePng
  convertToWebp
}

function deployDatabase {
  echo "Deploying database"
  cd /root/Jaylapp/Database
  bash merger.sh
  if [ -f "./merge.sql" ]; then
    systemctl start mysqld
    mysql -uroot -p${DB_PASSWORD} < merge.sql
    systemctl stop mysqld
  fi
  rm merge.sql
  cd /root
}

function deployWebclient {
  echo "Deploying webclient"
  cd /root/Jaylapp/Webclient
  npm install
  ng build --prod --aot
  html-minifier dist/Webclient/index.html --collapse-whitespace --remove-comments --remove-optional-tags --remove-redundant-attributes --remove-script-type-attributes --remove-tag-whitespace --use-short-doctype -o dist/Webclient/index.html
  cp /root/Jaylapp/Webclient/dist/Webclient/* /var/www/html/
  cd /root

  # Deploying optimized assets
  rm -rf /var/www/html/assets &> /dev/null
  cp -r /root/cache/assets /var/www/html/
}

function deployBackend {
  echo "Deploying backend"
  cd /root/Jaylapp/Backend
  rustup toolchain install nightly
  cargo update
  cargo build --release --all-features --jobs ${NUM_CORES}
  cargo install --path ./ --force
  cp /root/.cargo/bin/backend /home/yajla/
  cd /root
}

function stopServices {
  echo "Stopping services"
  systemctl stop nginx
  systemctl stop mysqld
  systemctl stop postfix
  systemctl stop backend
}

function startServices {
  echo "Starting services"
  systemctl start nginx
  systemctl start mysqld
  systemctl start postfix
  systemctl start backend
}

function deploy {
  pacman -Syu --noconfirm

  cd /root/Jaylapp
  git pull
  cd /root

  optimizeAssets

  stopServices
  certbot renew

  deployDatabase
  deployWebclient
  deployBackend

  startServices
}

deploy