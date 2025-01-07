#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum ThinBorders {
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
    NES,
    NEW,
    NSW,
    ESW,
    NESW,
}

const THIN_BORDER_LOOKUP: [char; 11] = ['└', '│', '┘', '┌', '─', '┐', '├', '┴', '┤', '┬', '┼'];

impl From<ThinBorders> for char {
    fn from(value: ThinBorders) -> Self {
        THIN_BORDER_LOOKUP[value as usize]
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum ThickBorders {
    NE,
    NS,
    NW,
    ES,
    EW,
    SW,
    NES,
    NEW,
    NSW,
    ESW,
    NESW,
}

const THICK_BORDER_LOOKUP: [char; 11] = ['┗', '┃', '┛', '┏', '━', '┓', '┣', '┻', '┫', '┳', '╋'];

impl From<ThickBorders> for char {
    fn from(value: ThickBorders) -> Self {
        THICK_BORDER_LOOKUP[value as usize]
    }
}
