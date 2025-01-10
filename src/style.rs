const THICK_BORDER_LOOKUP: [char; 16] = [
    ' ', '╹', '╺', '┗', '╻', '┃', '┏', '┣', '╸', '┛', '━', '┻', '┓', '┫', '┳', '╋',
];

pub fn thick_border_from_directions(north: bool, east: bool, south: bool, west: bool) -> char {
    let index = (north as usize)
        | ((east as usize) << 1)
        | ((south as usize) << 2)
        | ((west as usize) << 3);
    THICK_BORDER_LOOKUP[index]
}
