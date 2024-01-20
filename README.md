# ES Shipper

![GitHub last commit (branch)](https://img.shields.io/github/last-commit/n0xy1/suricata_es_shipper/main) ![GitHub commit activity](https://img.shields.io/github/commit-activity/w/n0xy1/suricata_es_shipper)

Ships suricata eve.json logs directly into elasticsearch.

## Config

It requires a config.yaml file with the following fields:

```
index: "suricata"
file_to_monitor: "/var/log/suricata/eve.json"
api_id: "YOUR_API_ID_HERE"
api_key: "YOUR_API_KEY_HERE"
es_url: "https://URI_OF_ELASTICSEARCH/"
```

If the index does not exist, it will create it within elasticsearch.

For simplicity I created the `config.yaml` within `/etc/es_shipper`. It can be changed within the rc script.

# FreeBSD rc.d

The `es_shipper.sh` file can be placed into: `/usr/local/etc/rc.d/es_shipper` 

Add a line like `es_shipper_enable="YES"` in /etc/rc.conf to enable automatic startup of your service. This can be done manually or using the sysrc command for a safer approach (e.g., `sysrc es_shipper_enable="YES"`).



## Disclaimer

**Use at Your Own Risk:** This software is provided "as is", without warranty of any kind. While every effort has been made to ensure functionality, the author(s) of this software do not guarantee its stability, security, or freedom from defects. The entire risk as to the quality and performance of the software is with the user.

**Potential Bugs and Security Vulnerabilities:** This software may contain bugs, errors, or security vulnerabilities. It has not undergone extensive testing and may not be suitable for use in critical or production environments where stability, security, and performance are crucial.

**No Liability:** In no event will the author(s) be held liable for any damages arising from the use of this software. Users are advised to review the code and conduct their own testing before deploying it in any environment.
