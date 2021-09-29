// One inch equates to 914400 EMUs and a centimeter is 360000 one pixel equates to 9525
type Emu = u32;

pub fn to_px(v: Emu) -> u32 {
    v / 9525
}

pub fn from_px(v: Emu) -> u32 {
    v * 9525
}
