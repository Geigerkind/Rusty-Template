Vagrant.configure("2") do |config|
  config.vm.box = "generic/arch"
  config.vm.box_check_update = true
  config.vm.hostname = "jaylapp"

  config.vm.provider "virtualbox" do |vb|
    vb.gui = false
    vb.memory = "4096"
    vb.cpus = `#{RbConfig::CONFIG['host_os'] =~ /darwin/ ? 'sysctl -n hw.ncpu' : 'nproc'}`.chomp
  end

  config.vm.network :private_network, ip: "10.10.10.10"
  config.vm.network "forwarded_port", guest: 80, guest_ip: "127.0.0.1", host: 8080
  config.vm.network "forwarded_port", guest: 443, guest_ip: "127.0.0.1", host: 8081

  config.vm.synced_folder "./", "/me", type: "rsync", rsync__auto: true, rsync__exclude: ['./Frontend/.*', './Backend/target/', './Backend/target/*']

  config.vm.provision "Packages",   type: "shell", run: "once", path: "Vagrant/packages.sh"
  config.vm.provision "Zopfli",     type: "shell", run: "once", path: "Vagrant/zopfli.sh"
  config.vm.provision "Database",   type: "shell", run: "once", path: "Vagrant/database.sh"
  config.vm.provision "Nginx",      type: "shell", run: "once", path: "Vagrant/nginx.sh"
  config.vm.provision "Frontend",   type: "shell", run: "once", path: "Vagrant/frontend.sh"
end
