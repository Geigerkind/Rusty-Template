## Installation
1. Install **docker**, **docker-compose**, **rustup**
2. Using rustup, install the **nightly** toolchain and set it to default
3. Make sure that no service is running on the following ports: 5555, 443, 80, 25, 4200 and 8000
4. Go into the Environment directory and start it using **docker-compose up**. (If you want to run it as daemon, append -d)
5. Go into the Backend directory and start the server using **cargo run**
6. Go into the Webclient directory and start the webclient using **ng serve**

## Enabling HTTPS locally
1. Import **Environment/nginx/ca.pem** as **authority** into your browser
2. Append the entry **jaylapp.dev 127.0.0.1** to your **hosts file**
3. Access the website using **jaylapp.dev** in your browser
