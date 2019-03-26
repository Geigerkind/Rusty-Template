# Initial DB install
mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql;

# Starting the database
systemctl start mysqld;

# Setting debug password
mysql -u root mysql -e 'UPDATE `user` SET password=PASSWORD("vagrant") WHERE user="root"';

# Applying new settings
systemctl restart mysqld;

# TODO: Once we have a structure, import it here!
bash /me/Database/merger.sh;

mysql -u root -pvagrant < /me/Database/merge.sql;
rm -f /me/Database/merge.sql;