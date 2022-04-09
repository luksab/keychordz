# QMK firmware
To build QMK for Keychordz, install qmk and clone the repository.
Then copy the `keychordz` folder to the `keyboards` folder in the qmk repository.

Now you can compile using 
```
qmk compile -kb keychordz -km default
```