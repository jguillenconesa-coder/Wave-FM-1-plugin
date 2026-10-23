// Copyright (c) 2026 Wave-FM-1-plugin contributors. MIT.
use crate::panel::{Palette, GRAPHITE, CREAM};

pub struct Skin { pub id: &'static str, pub name: &'static str, pub palette: Palette }
pub static SKINS: &[Skin] = &[
    Skin { id: "graphite", name: "Graphite / rose", palette: GRAPHITE },
    Skin { id: "cream", name: "Cream / mint", palette: CREAM },
];

pub fn get(id: &str) -> &'static Skin {
    SKINS.iter().find(|s| s.id == id).unwrap_or(&SKINS[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn skins_change_the_case_and_keys_without_recolouring_the_lcd() {
        let lcd = vec![0xf81f; 240 * 240];
        let st = crate::panel::State { lcd: &lcd, enc_clicks: &[0; 7], pot: 900,
            down: &[false; 41], leds: &[0.0; 41], turning: &[0; 8] };
        let mut images = Vec::new();
        for skin in SKINS {
            let base = crate::panel::base_with_palette(&skin.palette);
            let mut image = base.clone();
            crate::panel::draw_with_palette(&mut image, &base, &st, &skin.palette);
            assert_eq!(image[crate::panel::LCD_Y * crate::panel::W + crate::panel::LCD_X], 0xff00ff);
            images.push(image);
        }
        assert_ne!(images[0], images[1]);
        assert_eq!(get("unknown-future-skin").id, "graphite");
    }
}
