# QMK firmware
To build QMK for Keychordz, install qmk and clone the repository.
Then link the `keychordz` folder to the `keyboards` folder in the qmk repository.
```
ln -s $(pwd)/keychordz ~/qmk_firmware/keyboards/keychordz
```

Now you can compile using 
```
qmk compile -kb keychordz -km default
```