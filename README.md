# ORIGINAL

https://github.com/mathoudebine/turing-smart-screen-python

## Maybe you need to

```bash
sudo nano /etc/udev/rules.d/100-own.rules

SUBSYSTEM=="usb", ATTR{idVendor}=="1a86", ATTR{idProduct}=="5722", MODE="0666", GROUP="plugdev"

sudo udevadm control --reload-rules && sudo udevadm trigger
```

```bash
sudo usermod -aG dialout $USER
```