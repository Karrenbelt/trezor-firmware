/*
 * This file is part of the Trezor project, https://trezor.io/
 *
 * Copyright (c) SatoshiLabs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#pragma once

#include <trezor_types.h>

#ifdef KERNEL_MODE

// Initializes the touch driver
//
// Powers on and initializes touch driver controller.
// The function has no effect if the driver was already initialized.
//
// Returns `sectrue` if the hardware was successfully initialized.
secbool touch_init(void);

// Deinitializes the touch driver
//
// The function deinitializes touch controller and powers it off.
void touch_deinit();

// Powers on/off the touch controller
//
// The function is used to test touch power supply during production.
// After forcing power off, the touch controller needs to be re-initialized
void touch_power_set(bool on);

// Checks if the touch driver is ready to report touches
//
// Some drivers need time after power-up to stabilize. The app
// may use this function to wait until touch controller is
// fully functional.
secbool touch_ready(void);

// Gets the touch controller firmware version
//
// Can be called only if the touch controller was initialized,
// otherwise returns 0.
//
// We do not interpret the value of the version, we just print it
// during the production test.
uint8_t touch_get_version(void);

// Sets touch controller sensitivity
//
// (Internally threshold for ????)
secbool touch_set_sensitivity(uint8_t value);

// Checks if the touch is currently reporting any events
//
// The purpose of this function is very special. It is used
// in bootloader startup to detect if the user is touching the screen.
// On some hardware it's a bit more sensitive then `touch_get_event()`
// since it does not filter out any events.
//
// The function should not be used together with `touch_get_event()`.
secbool touch_activity(void);

#endif  // KERNEL_MODE

// Returns the last event in packed 32-bit format
//
// Returns `0` if there's no event or the driver is not initialized.
uint32_t touch_get_event(void);

// Touch event is packed 32-bit value
//
//  31    24 23        12 11         0
// |--------|------------|------------|
// |  event |    x-coord |    y-coord |
// |--------|------------|------------|
//
//

// Touch event bits
#define TOUCH_START (1U << 24)
#define TOUCH_MOVE (1U << 25)
#define TOUCH_END (1U << 26)
#define TOUCH_EVENT_MASK (0xFF << 24)

// Returns x-coordinates from a packed touch event
static inline uint16_t touch_unpack_x(uint32_t evt) {
  return (evt >> 12) & 0xFFF;
}

// Returns y-coordinates from a packed touch event
static inline uint16_t touch_unpack_y(uint32_t evt) {
  return (evt >> 0) & 0xFFF;
}

// Creates packed touch event from x and y coordinates
static inline uint32_t touch_pack_xy(uint16_t x, uint16_t y) {
  return ((x & 0xFFF) << 12) | (y & 0xFFF);
}
