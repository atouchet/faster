// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

macro_rules! impl_popcnt {
    ($($vec:ty, $fn:ident),*) => {
        $(
            impl Popcnt for $vec {
                #[inline(always)]
                #[allow(unused_unsafe)]
                fn count_ones(&self) -> usize {
                    unsafe { $fn(self.be_u8s()) }
                }
            }
        )*
    }
}