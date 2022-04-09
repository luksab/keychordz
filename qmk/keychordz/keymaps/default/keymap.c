#include QMK_KEYBOARD_H

const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
     /*
      * ┌───┬───┬───┬───┐       ┌───┬───┬───┬───┐
      * │ A │ S │ D │ F │       │ H │ J │ K │ L │
      * └───┴───┴───┴───┘       └───┴───┴───┴───┘
      *              ┌───┐   ┌───┐
      *          ┌───┤Alt│   │Alt├───┐
      *          │Bsp├───┤   ├───┤Bsp│
      *          └───┤Tab│   │Spc├───┘
      *              └───┘   └───┘
      */
    [0] = LAYOUT(
        KC_A,    KC_S,    KC_D,    KC_F,                               KC_H,    KC_J,    KC_K,    KC_L,
                       KC_BSPC, KC_LALT, KC_TAB,           KC_SPC, KC_RALT, KC_ENT
    )
};
