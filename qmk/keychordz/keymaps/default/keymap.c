#include QMK_KEYBOARD_H
#include "keychordz.h"

const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
     /*
      * ┌───┬───┬───┬───┐       ┌───┬───┬───┬───┐
      * │000│001│002│003│       │023│022│021│020│
      * └───┴───┴───┴───┘       └───┴───┴───┴───┘
      *
      *          ┌───┐             ┌───┐   
      *          │010├───┐     ┌───┤030│   
      *          ├───┤012│     │032├───┤   
      *          │011├───┘     └───┤031│   
      *          └───┘             └───┘   
      */
    [0] = LAYOUT_keychordz(
        KC_A,    KC_S,    KC_D,    KC_F,                               KC_H,    KC_J,    KC_K,    KC_L,  \
                       KC_BSPC, KC_LALT, KC_TAB,           KC_SPC, KC_RALT, KC_ENT  \
    )
};
