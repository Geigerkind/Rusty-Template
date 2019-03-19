# Creating www directory
mkdir -p /var/www;

# Giving it user accessible rights
chmod -R 777 /var/www;

# Compiling the frontend
bash /me/Frontend/compile.sh debug false;