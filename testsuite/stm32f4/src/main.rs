// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! OS test suite, STM32F407 wrapper.

#![no_std]
#![no_main]

// get the panic handler
use panic_semihosting as _;

/// This constant assumes a 16MHz clock at reset. You probably don't need to
/// adjust it if your processor starts at a different speed; none of the tests
/// rely on this being _correct,_ they only require it to have been set.
const HZ: u32 = 16_000_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    lilos_testsuite::run_test_suite(HZ)
}
