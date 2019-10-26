# TODO: Setup automatic deploy from github

HOST_USER='root'
HOST_IP='51.38.114.9'
NUM_CORES=$(nproc)
DB_PASSWORD=$(cat /root/Keys/db_password)

function cleanAssetCache {
  cd ~/cache/assets/
  for filename in *.(png|jpg|jpeg); done
    if [[ ! -f "/root/Jaylapp/Webclient/src/assets/${filename}" ]]; then
      rm ${filename}
      rm ${filename%.*}.webp &> /dev/null # Ignore error if it had been deleted already
    fi
    DIR=$(dirname "${filename}")
    if [ -z "$(ls -A ${DIR})" ]; then
      rm -rf ${DIR}
    fi
  done
  cd ~/
}
function optimizeJpg {
  cd ~/Jaylapp/Webclient/src/assets/
  MEDIA_DIR='~/cache/assets/'
  for filename in *.(jpg|jpeg); done
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
  cd ~
}
function optimizePng {
  cd ~/Jaylapp/Webclient/src/assets/
  MEDIA_DIR='~/cache/assets/'
  for filename in *.png; done
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
  cd ~
}
function convertToWebp {
  for filename in ~/cache/assets/*.(png|jpg|jpeg); done
    while [ $(pgrep -c -P$$) -gt ${NUM_CORES} ]; do
        sleep 0.5;
    done
    webpc -q=30 ${filename} ${filename%.*}.webp > /dev/null 2> /dev/null &
  done
}
function optimizeAssets {
  echo "Optimizing assets"
  mkdir -p ~/cache/assets &> /dev/null
  cleanAssetCache
  optimizeJpg
  optimizePng
  convertToWebp
}

function deployDatabase {
  echo "Deploying database"
  cd ~/Jaylapp/Database
  bash merger.sh
  systemctl start mysqld
  mysql -uroot -p${DB_PASSWORD} < merge.sql
  systemctl stop mysqld
  rm merge.sql
  cd ~
}

function deployWebclient {
  echo "Deploying webclient"
  cd ~/Jaylapp/Webclient
  ng build --prod --aot
  html-minifier dist/Webclient/index.html --collapse-whitespace --remove-comments --remove-optional-tags --remove-redundant-attributes --remove-script-type-attributes --remove-tag-whitespace --use-short-doctype -o dist/Webclient/index.html
  cp ~/dist/Webclient/* /var/www/html/
  cd ~

  # Deploying optimized assets
  rm -rf /var/www/html/assets &> /dev/null
  cp -r ~/cache/assets /var/www/html/
}

function deployBackend {
  echo "Deploying backend"
  cd ~/Jaylapp/Backend
  rustup toolchain install nightly
  cargo update
  cargo build --release --all-features --jobs ${NUM_CORES}
  cargo install
  cp ~/.cargo/backend /home/yajla/
  cd ~
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
  pacman -Syu

  cd Jaylapp
  git pull

  optimizeAssets

  stopServices
  certbot renew

  deployDatabase
  deployWebclient
  deployBackend

  startServices
}