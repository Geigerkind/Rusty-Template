# Generating certificates
mkdir /etc/nginx/cert;

# DH param
cp /me/Vagrant/nginx/dhparam.pem /etc/nginx/cert/dhparam.pem;

# Self-Signed certificate for dev
# To generate it see: https://stackoverflow.com/questions/50788043/how-to-trust-self-signed-localhost-certificates-on-linux-chrome-and-firefox
cp /me/Vagrant/nginx/jaylapp.crt /etc/nginx/cert/;
cp /me/Vagrant/nginx/jaylapp.key.pem /etc/nginx/cert/;

# Importing config file
cp /me/Vagrant/nginx/nginx.conf /etc/nginx/;

# Starting nginx
systemctl start nginx;