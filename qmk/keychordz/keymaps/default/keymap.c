#include QMK_KEYBOARD_H
#include "keychordz.h"

#define PHY_NUMBER_OF_COLS 12
#define PHY_NUMBER_OF_ROWS 3

#define LED_X(x) 224 / (PHY_NUMBER_OF_COLS - 1) * x
#define LED_Y(y)  64 / (PHY_NUMBER_OF_ROWS - 1) * y

#define LED_POS(x, y) { LED_X(x), LED_Y(y) }

led_config_t g_led_config = { {
  // Key Matrix to LED Index
  // Left
  {   0, 1, 2, 3 },
  {   4, 5, 6, NO_LED }, // Right
                        {   7, 8, 9, 10 },
                        {  11, 12, 13, NO_LED }
}, {
  // LED Index to Physical Position
  LED_POS(0, 0), LED_POS(1, 0), LED_POS(2, 0), LED_POS(3, 0),
  LED_POS(5, 1), LED_POS(4, 2), LED_POS(5, 2),
  LED_POS(12, 0), LED_POS(11, 0), LED_POS(10, 0), LED_POS(9, 0),
  LED_POS(7, 1), LED_POS(8, 2), LED_POS(7, 2)
}, {
  // LED Index to Flag
  4, 4, 4, 4,
  4, 4, 4,
  4, 4, 4, 4,
  4, 4, 4
} };


const uint16_t PROGMEM combo_14[] = { KC_T, KC_E, COMBO_END};
const uint16_t PROGMEM combo_15[] = { KC_T, KC_S, COMBO_END};
const uint16_t PROGMEM combo_16[] = { KC_T, KC_A, COMBO_END};
const uint16_t PROGMEM combo_17[] = { KC_T, KC_N, COMBO_END};
const uint16_t PROGMEM combo_18[] = { KC_T, KC_I, COMBO_END};
const uint16_t PROGMEM combo_19[] = { KC_T, KC_O, COMBO_END};
const uint16_t PROGMEM combo_20[] = { KC_T, KC_P, COMBO_END};
const uint16_t PROGMEM combo_21[] = { KC_N, KC_I, KC_O, COMBO_END};
const uint16_t PROGMEM combo_22[] = { KC_N, KC_I, KC_P, COMBO_END};
const uint16_t PROGMEM combo_23[] = { KC_N, KC_I, KC_T, COMBO_END};

combo_t key_combos[COMBO_COUNT] = {
    COMBO(combo_14, KC_R),
    COMBO(combo_15, KC_C),
    COMBO(combo_16, KC_F),
    COMBO(combo_17, KC_B),
    COMBO(combo_18, KC_V),
    COMBO(combo_19, KC_G),
    COMBO(combo_20, KC_BSPC),
    COMBO(combo_21, RGB_TOG),
    COMBO(combo_22, RGB_MODE_FORWARD),
    COMBO(combo_23, RGB_VAI),
};

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
       KC_A,    KC_S,    KC_E,    KC_T,                      KC_N,    KC_I,    KC_O,    KC_P, \
                             KC_TAB, KC_SPACE, KC_LEFT_SHIFT,            KC_N, KC_N, KC_N \
       )
};

// [L/R] [Pinkie, Ring, Middle, Index, thumbL, thumbU, thumbD]
// #define K_0_LP KC_A
// #define K_0_LR KC_S
// #define K_0_LM KC_D
// #define K_0_LI KC_F
// #define K_0_LL KC_G
// #define K_0_LU KC_H
// #define K_0_LD KC_J
// #define K_0_RP KC_K
// #define K_0_RR KC_L
// #define K_0_RM KC_M
// #define K_0_RI KC_N
// #define K_0_RL KC_O
// #define K_0_RU KC_P
// #define K_0_RD KC_Q
