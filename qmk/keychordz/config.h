// Copyright 2022 Lukas Sabatschus (@luksab)
// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once

#include "config_common.h"

#define EE_HANDS
// #define SPLIT_HAND_PIN B4
#define USE_I2C
#define SPLIT_USB_DETECT

#define MATRIX_ROWS 4
#define MATRIX_COLS 4

#define DIRECT_PINS { { C6, D4, D7, E6 }, { B3, B2, B1, NO_PIN } }

#define RGB_DI_PIN B5
#define DRIVER_LED_TOTAL 14
#define RGB_MATRIX_SPLIT { 7, 7 }
#define RGB_MATRIX_KEYPRESSES
#define SPLIT_TRANSPORT_MIRROR
#define SPLIT_LAYER_STATE_ENABLE
#define SPLIT_MODS_ENABLE

#define ENABLE_RGB_MATRIX_CYCLE_LEFT_RIGHT
#define ENABLE_RGB_MATRIX_SOLID_COLOR
#define ENABLE_RGB_MATRIX_BREATHING
#define ENABLE_RGB_MATRIX_CYCLE_LEFT_RIGHT
// #define ENABLE_RGB_MATRIX_PIXEL_FLOW
// #define ENABLE_RGB_MATRIX_SOLID_REACTIVE_MULTIWIDE
#define ENABLE_RGB_MATRIX_SPLASH
#define ENABLE_RGB_MATRIX_MULTISPLASH

#define ENABLE_RGB_MATRIX_SOLID_REACTIVE_SIMPLE
#define RGB_MATRIX_TYPING_HEATMAP_DECREASE_DELAY_MS 50

#define RGB_MATRIX_MAXIMUM_BRIGHTNESS 255

#define RGB_MATRIX_STARTUP_MODE RGB_MATRIX_SOLID_COLOR
#define RGB_MATRIX_STARTUP_VAL 32

#define RGB_MATRIX_KEYPRESSES

#define COMBO_TERM 1000
#define COMBO_COUNT 31

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
