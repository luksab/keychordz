// Copyright 2022 Lukas Sabatschus (@luksab)
// SPDX-License-Identifier: GPL-2.0-or-later

#pragma once

// #define EE_HANDS
#define SPLIT_HAND_PIN B4
#define USE_I2C
#define SPLIT_USB_DETECT

#define MATRIX_ROWS 4
#define MATRIX_COLS 4

#define DIRECT_PINS { { C6, D4, D7, E6 }, { B3, B2, B1, NO_PIN } }

#define COMBO_TERM 1000
#define COMBO_COUNT 0

/*
 * Feature disable options
 *  These options are also useful to firmware size reduction.
 */

/* disable debug print */
//#define NO_DEBUG

/* disable print */
//#define NO_PRINT

/* disable action features */
//#define NO_ACTION_LAYER
//#define NO_ACTION_TAPPING
//#define NO_ACTION_ONESHOT
