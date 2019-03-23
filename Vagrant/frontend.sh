# Creating www directory
mkdir -p /var/www;

# Compiling the frontend
bash /me/Frontend/compile.sh debug false;

# Giving it user accessible rights
chmod -R 777 /var/www;
chown -R vagrant:root /var/www;