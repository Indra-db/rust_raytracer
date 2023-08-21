use glam::Vec3;
use std::collections::HashMap;

pub type RGBColor = Vec3;

pub fn create_linear_fresnel_hash_map() -> HashMap<&'static str, RGBColor> {
    let mut map = HashMap::new();
    map.insert("Titanium", RGBColor { x: 0.542, y: 0.497, z: 0.449 });
    map.insert("Chrome", RGBColor { x: 0.549, y: 0.556, z: 0.554 });
    map.insert("Iron", RGBColor { x: 0.562, y: 0.565, z: 0.578 });
    map.insert("Nickel", RGBColor { x: 0.660, y: 0.609, z: 0.526 });
    map.insert("Platinum", RGBColor { x: 0.673, y: 0.637, z: 0.585 });
    map.insert("Copper", RGBColor { x: 0.955, y: 0.638, z: 0.538 });
    map.insert("Palladium", RGBColor { x: 0.733, y: 0.697, z: 0.652 });
    map.insert("Mercury", RGBColor { x: 0.781, y: 0.780, z: 0.778 });
    map.insert("Brass", RGBColor { x: 0.910, y: 0.778, z: 0.423 });
    map.insert("Zinc", RGBColor { x: 0.664, y: 0.824, z: 0.850 });
    map.insert("Gold", RGBColor { x: 1.000, y: 0.782, z: 0.344 });
    map.insert("Aluminum", RGBColor { x: 0.913, y: 0.922, z: 0.924 });
    map.insert("Silver", RGBColor { x: 0.972, y: 0.960, z: 0.915 });
    map.insert("Dielectric", RGBColor { x: 0.040, y: 0.040, z: 0.040 });

    map
}

pub fn create_diffuse_rgb_hash_map() -> HashMap<&'static str, RGBColor> {
    let mut map = HashMap::new();

    map.insert("AliceBlue", RGBColor { x: 240.0, y: 248.0, z: 255.0 });
    map.insert("AntiqueWhite", RGBColor { x: 250.0, y: 235.0, z: 215.0 });
    map.insert("Aqua", RGBColor { x: 0.0, y: 255.0, z: 255.0 });
    map.insert("AquaMarine", RGBColor { x: 127.0, y: 255.0, z: 212.0 });
    map.insert("Azure", RGBColor { x: 240.0, y: 255.0, z: 255.0 });
    map.insert("Beige", RGBColor { x: 245.0, y: 245.0, z: 220.0 });
    map.insert("Bisque", RGBColor { x: 255.0, y: 228.0, z: 196.0 });
    map.insert("Black", RGBColor { x: 0.0, y: 0.0, z: 0.0 });
    map.insert("BlancheDalmond", RGBColor { x: 255.0, y: 235.0, z: 205.0 });
    map.insert("Blue", RGBColor { x: 0.0, y: 0.0, z: 255.0 });
    map.insert("BlueViolet", RGBColor { x: 138.0, y: 43.0, z: 226.0 });
    map.insert("Brown", RGBColor { x: 165.0, y: 42.0, z: 42.0 });
    map.insert("BurlyWood", RGBColor { x: 222.0, y: 184.0, z: 135.0 });
    map.insert("CadetBlue", RGBColor { x: 95.0, y: 158.0, z: 160.0 });
    map.insert("ChartReuse", RGBColor { x: 127.0, y: 255.0, z: 0.0 });
    map.insert("Chocolate", RGBColor { x: 210.0, y: 105.0, z: 30.0 });
    map.insert("Coral", RGBColor { x: 255.0, y: 127.0, z: 80.0 });
    map.insert("CornFlowerBlue", RGBColor { x: 100.0, y: 149.0, z: 237.0 });
    map.insert("Cornsilk", RGBColor { x: 255.0, y: 248.0, z: 220.0 });
    map.insert("Crimson", RGBColor { x: 220.0, y: 20.0, z: 60.0 });
    map.insert("Cyan", RGBColor { x: 0.0, y: 255.0, z: 255.0 });
    map.insert("DarkBlue", RGBColor { x: 0.0, y: 0.0, z: 139.0 });
    map.insert("DarkCyan", RGBColor { x: 0.0, y: 139.0, z: 139.0 });
    map.insert("DarkGoldenrod", RGBColor { x: 184.0, y: 134.0, z: 11.0 });
    map.insert("DarkGray", RGBColor { x: 169.0, y: 169.0, z: 169.0 });
    map.insert("DarkGreen", RGBColor { x: 0.0, y: 100.0, z: 0.0 });
    map.insert("DarkGrey", RGBColor { x: 169.0, y: 169.0, z: 169.0 });
    map.insert("DarkKhaki", RGBColor { x: 189.0, y: 183.0, z: 107.0 });
    map.insert("DarkMagenta", RGBColor { x: 139.0, y: 0.0, z: 139.0 });
    map.insert("DarkOliveGreen", RGBColor { x: 85.0, y: 107.0, z: 47.0 });
    map.insert("DarkOrange", RGBColor { x: 255.0, y: 140.0, z: 0.0 });
    map.insert("DarkOrchid", RGBColor { x: 153.0, y: 50.0, z: 204.0 });
    map.insert("DarkRed", RGBColor { x: 139.0, y: 0.0, z: 0.0 });
    map.insert("DarkSalmon", RGBColor { x: 233.0, y: 150.0, z: 122.0 });
    map.insert("DarkSeaGreen", RGBColor { x: 143.0, y: 188.0, z: 143.0 });
    map.insert("DarkSlateBlue", RGBColor { x: 72.0, y: 61.0, z: 139.0 });
    map.insert("DarkSlateGray", RGBColor { x: 47.0, y: 79.0, z: 79.0 });
    map.insert("DarkSlateGrey", RGBColor { x: 47.0, y: 79.0, z: 79.0 });
    map.insert("DarkTurquoise", RGBColor { x: 0.0, y: 206.0, z: 209.0 });
    map.insert("DarkViolet", RGBColor { x: 148.0, y: 0.0, z: 211.0 });
    map.insert("DeepPink", RGBColor { x: 255.0, y: 20.0, z: 147.0 });
    map.insert("DeepSkyBlue", RGBColor { x: 0.0, y: 191.0, z: 255.0 });
    map.insert("DimGray", RGBColor { x: 105.0, y: 105.0, z: 105.0 });
    map.insert("DimGrey", RGBColor { x: 105.0, y: 105.0, z: 105.0 });
    map.insert("DodgerBlue", RGBColor { x: 30.0, y: 144.0, z: 255.0 });
    map.insert("FireBrick", RGBColor { x: 178.0, y: 34.0, z: 34.0 });
    map.insert("FloralWhite", RGBColor { x: 255.0, y: 250.0, z: 240.0 });
    map.insert("ForestGreen", RGBColor { x: 34.0, y: 139.0, z: 34.0 });
    map.insert("Fuchsia", RGBColor { x: 255.0, y: 0.0, z: 255.0 });
    map.insert("Gainsboro", RGBColor { x: 220.0, y: 220.0, z: 220.0 });
    map.insert("GhostWhite", RGBColor { x: 248.0, y: 248.0, z: 255.0 });
    map.insert("Gold", RGBColor { x: 255.0, y: 215.0, z: 0.0 });
    map.insert("Goldenrod", RGBColor { x: 218.0, y: 165.0, z: 32.0 });
    map.insert("Gray", RGBColor { x: 128.0, y: 128.0, z: 128.0 });
    map.insert("Green", RGBColor { x: 0.0, y: 128.0, z: 0.0 });
    map.insert("GreenYellow", RGBColor { x: 173.0, y: 255.0, z: 47.0 });
    map.insert("Grey", RGBColor { x: 128.0, y: 128.0, z: 128.0 });
    map.insert("Honeydew", RGBColor { x: 240.0, y: 255.0, z: 240.0 });
    map.insert("HotPink", RGBColor { x: 255.0, y: 105.0, z: 180.0 });
    map.insert("IndianRed", RGBColor { x: 205.0, y: 92.0, z: 92.0 });
    map.insert("Indigo", RGBColor { x: 75.0, y: 0.0, z: 130.0 });
    map.insert("Ivory", RGBColor { x: 255.0, y: 255.0, z: 240.0 });
    map.insert("Khaki", RGBColor { x: 240.0, y: 230.0, z: 140.0 });
    map.insert("Lavender", RGBColor { x: 230.0, y: 230.0, z: 250.0 });
    map.insert("LavenderBlush", RGBColor { x: 255.0, y: 240.0, z: 245.0 });
    map.insert("LawnGreen", RGBColor { x: 124.0, y: 252.0, z: 0.0 });
    map.insert("LemonChiffon", RGBColor { x: 255.0, y: 250.0, z: 205.0 });
    map.insert("LightBlue", RGBColor { x: 173.0, y: 216.0, z: 230.0 });
    map.insert("LightCoral", RGBColor { x: 240.0, y: 128.0, z: 128.0 });
    map.insert("LightCyan", RGBColor { x: 224.0, y: 255.0, z: 255.0 });
    map.insert("LightGoldenrodYellow", RGBColor { x: 250.0, y: 250.0, z: 210.0 });
    map.insert("LightGray", RGBColor { x: 211.0, y: 211.0, z: 211.0 });
    map.insert("LightGreen", RGBColor { x: 144.0, y: 238.0, z: 144.0 });
    map.insert("LightGrey", RGBColor { x: 211.0, y: 211.0, z: 211.0 });
    map.insert("LightPink", RGBColor { x: 255.0, y: 182.0, z: 193.0 });
    map.insert("LightSalmon", RGBColor { x: 255.0, y: 160.0, z: 122.0 });
    map.insert("LightSeaGreen", RGBColor { x: 32.0, y: 178.0, z: 170.0 });
    map.insert("LightSkyBlue", RGBColor { x: 135.0, y: 206.0, z: 250.0 });
    map.insert("LightSlateGray", RGBColor { x: 119.0, y: 136.0, z: 153.0 });
    map.insert("LightSlateGrey", RGBColor { x: 119.0, y: 136.0, z: 153.0 });
    map.insert("LightSteelBlue", RGBColor { x: 176.0, y: 224.0, z: 230.0 });
    map.insert("LightYellow", RGBColor { x: 255.0, y: 255.0, z: 224.0 });
    map.insert("Lime", RGBColor { x: 0.0, y: 255.0, z: 0.0 });
    map.insert("LimeGreen", RGBColor { x: 50.0, y: 205.0, z: 50.0 });
    map.insert("Linen", RGBColor { x: 250.0, y: 240.0, z: 230.0 });
    map.insert("Magenta", RGBColor { x: 255.0, y: 0.0, z: 255.0 });
    map.insert("Maroon", RGBColor { x: 128.0, y: 0.0, z: 0.0 });
    map.insert("MediumAquaMarine", RGBColor { x: 102.0, y: 205.0, z: 170.0 });
    map.insert("MediumBlue", RGBColor { x: 0.0, y: 0.0, z: 205.0 });
    map.insert("MediumOrchid", RGBColor { x: 186.0, y: 85.0, z: 211.0 });
    map.insert("MediumPurple", RGBColor { x: 147.0, y: 112.0, z: 219.0 });
    map.insert("MediumSeaGreen", RGBColor { x: 60.0, y: 179.0, z: 113.0 });
    map.insert("MediumSlateBlue", RGBColor { x: 123.0, y: 104.0, z: 238.0 });
    map.insert("MediumSpringGreen", RGBColor { x: 0.0, y: 250.0, z: 154.0 });
    map.insert("MediumTurquoise", RGBColor { x: 72.0, y: 209.0, z: 204.0 });
    map.insert("MediumVioletRed", RGBColor { x: 199.0, y: 21.0, z: 133.0 });
    map.insert("MidnightBlue", RGBColor { x: 25.0, y: 25.0, z: 112.0 });
    map.insert("MintCream", RGBColor { x: 245.0, y: 255.0, z: 250.0 });
    map.insert("MistyRose", RGBColor { x: 255.0, y: 228.0, z: 225.0 });
    map.insert("Moccasin", RGBColor { x: 255.0, y: 228.0, z: 181.0 });
    map.insert("NavajoWhite", RGBColor { x: 255.0, y: 222.0, z: 173.0 });
    map.insert("Navy", RGBColor { x: 0.0, y: 0.0, z: 128.0 });
    map.insert("OldLace", RGBColor { x: 253.0, y: 245.0, z: 230.0 });
    map.insert("Olive", RGBColor { x: 128.0, y: 128.0, z: 0.0 });
    map.insert("OliveDrab", RGBColor { x: 107.0, y: 142.0, z: 35.0 });
    map.insert("Orange", RGBColor { x: 255.0, y: 165.0, z: 0.0 });
    map.insert("OrangeRed", RGBColor { x: 255.0, y: 69.0, z: 0.0 });
    map.insert("Orchid", RGBColor { x: 218.0, y: 112.0, z: 214.0 });
    map.insert("PaleGoldenrod", RGBColor { x: 238.0, y: 232.0, z: 170.0 });
    map.insert("PaleGreen", RGBColor { x: 152.0, y: 251.0, z: 152.0 });
    map.insert("PaleTurquoise", RGBColor { x: 175.0, y: 238.0, z: 238.0 });
    map.insert("PaleVioletRed", RGBColor { x: 219.0, y: 112.0, z: 147.0 });
    map.insert("PapayaWhip", RGBColor { x: 255.0, y: 239.0, z: 213.0 });
    map.insert("PeachPuff", RGBColor { x: 255.0, y: 218.0, z: 185.0 });
    map.insert("Peru", RGBColor { x: 205.0, y: 133.0, z: 63.0 });
    map.insert("Pink", RGBColor { x: 255.0, y: 192.0, z: 203.0 });
    map.insert("Plum", RGBColor { x: 221.0, y: 160.0, z: 221.0 });
    map.insert("PowderBlue", RGBColor { x: 176.0, y: 224.0, z: 230.0 });
    map.insert("Purple", RGBColor { x: 128.0, y: 0.0, z: 128.0 });
    map.insert("RebeccaPurple", RGBColor { x: 102.0, y: 51.0, z: 153.0 });
    map.insert("Red", RGBColor { x: 255.0, y: 0.0, z: 0.0 });
    map.insert("RosyBrown", RGBColor { x: 188.0, y: 143.0, z: 143.0 });
    map.insert("RoyalBlue", RGBColor { x: 65.0, y: 105.0, z: 225.0 });
    map.insert("SaddleBrown", RGBColor { x: 139.0, y: 69.0, z: 19.0 });
    map.insert("Salmon", RGBColor { x: 250.0, y: 128.0, z: 114.0 });
    map.insert("SandyBrown", RGBColor { x: 244.0, y: 164.0, z: 96.0 });
    map.insert("SeaGreen", RGBColor { x: 46.0, y: 139.0, z: 87.0 });
    map.insert("SeaShell", RGBColor { x: 255.0, y: 245.0, z: 238.0 });
    map.insert("Sienna", RGBColor { x: 160.0, y: 82.0, z: 45.0 });
    map.insert("Silver", RGBColor { x: 192.0, y: 192.0, z: 192.0 });
    map.insert("SkyBlue", RGBColor { x: 135.0, y: 206.0, z: 235.0 });
    map.insert("SlateBlue", RGBColor { x: 106.0, y: 90.0, z: 205.0 });
    map.insert("SlateGray", RGBColor { x: 112.0, y: 128.0, z: 144.0 });
    map.insert("SlateGrey", RGBColor { x: 112.0, y: 128.0, z: 144.0 });
    map.insert("Snow", RGBColor { x: 255.0, y: 250.0, z: 250.0 });
    map.insert("SpringGreen", RGBColor { x: 0.0, y: 255.0, z: 127.0 });
    map.insert("SteelBlue", RGBColor { x: 70.0, y: 130.0, z: 180.0 });
    map.insert("Tan", RGBColor { x: 210.0, y: 180.0, z: 140.0 });
    map.insert("Teal", RGBColor { x: 0.0, y: 128.0, z: 128.0 });
    map.insert("Thistle", RGBColor { x: 216.0, y: 191.0, z: 216.0 });
    map.insert("Tomato", RGBColor { x: 255.0, y: 99.0, z: 71.0 });
    map.insert("Turquoise", RGBColor { x: 64.0, y: 224.0, z: 208.0 });
    map.insert("Violet", RGBColor { x: 238.0, y: 130.0, z: 238.0 });
    map.insert("Wheat", RGBColor { x: 245.0, y: 222.0, z: 179.0 });
    map.insert("White", RGBColor { x: 255.0, y: 255.0, z: 255.0 });
    map.insert("WhiteSmoke", RGBColor { x: 245.0, y: 245.0, z: 245.0 });
    map.insert("Yellow", RGBColor { x: 255.0, y: 255.0, z: 0.0 });
    map.insert("YellowGreen", RGBColor { x: 154.0, y: 205.0, z: 50.0 });

    map
}
