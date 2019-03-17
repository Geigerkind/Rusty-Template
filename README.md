## How to contribute
1. Create a branch with the suffixes *"feature/", "fix/", "hotfix/" etc."
2. Do your changes
3. Create a pull request
4. Wait for someone else to review it

## Installation
1. Install **vagrant** and **virtualbox** (Windows user should prefer the Linux shell)
2. Import **Vagrant/nginx/ca.pem** as **authority** into your browser
3. Append the entry **jaylapp.dev 10.10.10.10** to your **hosts file**
4. Go into the root directory and type **vagrant up**

## Nett To Know
- Use **vagrant destroy -f** to destroy the vm
- Using **vagrant halt** does not require a full reinstallation after a **vagrant up**
