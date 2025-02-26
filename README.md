# DNS Updater

## Purpose
This project is created for homelab creators who use dynamic IP and cannot affort to buy static IP for themselves to start the project.

The idea is to use this under systemd service on any machine in the homelab cluster as long as it uses systemd and able to install `.deb` packages.


## Usage
Please take a look on .env.template file. Where you can fill in your DNS information (For now, this only supports CloudFlare and CloudFlare's Update Record API with one single record.)

```sh
API_KEY=<API-KEY> #This one is actually the public API key that you can find in your dashboard where `ZONE` also was
ZONE=<ZONE> #Take a look at your personal dashboard in a specific domain that you own, you would see it.
```

## Contribution

Feel free to download, rewrite it.

HOWEVER, DON'T COMMERCIALIZE IT. EXCEPT ME