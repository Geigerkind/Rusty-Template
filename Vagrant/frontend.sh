# Creating www directory
mkdir -p /var/www;

# Debug: Hello World
mkdir /var/www/jaylapp;
echo "Hello World!" > /var/www/jaylapp/index.html;

# Giving it user accessible rights
chmod -R 777 /var/www;

# Compiling the frontend
# bash /me/Frontend/compile.sh debug false;