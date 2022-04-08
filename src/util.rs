// Copyright (c) 2022 Jan Holthuis
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Common types used in multiple modules.

use binrw::binrw;

/// Indexed Color identifiers used for memory cues and tracks.
#[binrw]
#[derive(Debug, PartialEq, Clone)]
pub enum ColorIndex {
    /// No color.
    #[brw(magic = 0u8)]
    None,
    /// Pink color.
    #[brw(magic = 1u8)]
    Pink,
    /// Red color.
    #[brw(magic = 2u8)]
    Red,
    /// Orange color.
    #[brw(magic = 3u8)]
    Orange,
    /// Yellow color.
    #[brw(magic = 4u8)]
    Yellow,
    /// Green color.
    #[brw(magic = 5u8)]
    Green,
    /// Aqua color.
    #[brw(magic = 6u8)]
    Aqua,
    /// Blue color.
    #[brw(magic = 7u8)]
    Blue,
    /// Purple color.
    #[brw(magic = 8u8)]
    Purple,
    /// Unknown color.
    Unknown(u16),
}

impl From<u16> for ColorIndex {
    fn from(color_id: u16) -> Self {
        match color_id {
            0 => Self::None,
            1 => Self::Pink,
            2 => Self::Red,
            3 => Self::Orange,
            4 => Self::Yellow,
            5 => Self::Green,
            6 => Self::Aqua,
            7 => Self::Blue,
            8 => Self::Purple,
            x => Self::Unknown(x),
        }
    }
}

#[cfg(test)]
pub(in crate) mod testing {
    use binrw::prelude::*;
    pub fn test_roundtrip<T>(bin: &[u8], obj: T)
    where
        <T as binrw::BinRead>::Args: Default,
        <T as binrw::BinWrite>::Args: Default,
        T: BinRead + BinWrite + PartialEq + core::fmt::Debug,
    {
        // T->binary
        let mut writer = binrw::io::Cursor::new(Vec::with_capacity(bin.len()));
        obj.write_to(&mut writer).unwrap();
        assert_eq!(bin, writer.get_ref());
        // T->binary->T
        writer.set_position(0);
        let parsed = T::read(&mut writer).unwrap();
        assert_eq!(obj, parsed);
        // binary->T
        let mut cursor = binrw::io::Cursor::new(bin);
        let parsed = T::read(&mut cursor).unwrap();
        assert_eq!(obj, parsed);
        // binary->T->binary
        writer.set_position(0);
        parsed.write_to(&mut writer).unwrap();
        assert_eq!(bin, writer.get_ref());
    }
}
