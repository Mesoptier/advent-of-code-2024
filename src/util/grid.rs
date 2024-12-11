pub type Coord = (usize, usize);

pub trait Grid {
    type Item;

    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, coord: Coord) -> Option<&Self::Item>;

    fn iter(&self) -> impl Iterator<Item = (Coord, &Self::Item)> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .map(move |coord| (coord, self.get(coord).unwrap()))
    }
}

pub trait GridMut: Grid {
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Self::Item>;
}

macro_rules! index_impl {
    ($t:ty, $($args:tt)*) => {
        impl<$($args,)*> std::ops::Index<Coord> for $t {
            type Output = <Self as Grid>::Item;

            fn index(&self, coord: Coord) -> &Self::Output {
                self.get(coord).unwrap()
            }
        }
    };
}

macro_rules! index_mut_impl {
    ($t:ty, $($args:tt)*) => {
        impl<$($args,)*> std::ops::IndexMut<Coord> for $t {
            fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
                self.get_mut(coord).unwrap()
            }
        }
    };
}

/// Wraps an LF-separated multiline ASCII string with equal line widths such that it can be
/// addressed like a 2-dimensional grid of bytes.
pub struct StrGrid<'a> {
    line_width: usize,
    data: &'a [u8],
}

impl<'a> StrGrid<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let line_width = data.iter().position(|&c| c == b'\n').unwrap() + 1;
        assert_eq!(data.len() % line_width, 0);
        Self { line_width, data }
    }

    fn coord_to_data_index(&self, (x, y): Coord) -> Option<usize> {
        (x < self.width() && y < self.height()).then(|| x + (y * self.line_width))
    }
}

impl Grid for StrGrid<'_> {
    type Item = u8;

    fn width(&self) -> usize {
        self.line_width - 1
    }

    fn height(&self) -> usize {
        self.data.len() / self.line_width
    }

    fn get(&self, coord: Coord) -> Option<&u8> {
        self.coord_to_data_index(coord).map(|index| unsafe {
            // SAFETY: `index` is guaranteed to be valid.
            self.data.get_unchecked(index)
        })
    }
}

index_impl! (StrGrid<'a>, 'a);

pub struct VecGrid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> VecGrid<T> {
    pub fn from_data(width: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len() % width, 0);
        Self { data, width }
    }

    fn coord_to_data_index(&self, (x, y): Coord) -> Option<usize> {
        (x < self.width() && y < self.height()).then(|| x + (y * self.width))
    }
}

impl<T> Grid for VecGrid<T> {
    type Item = T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn get(&self, coord: Coord) -> Option<&Self::Item> {
        self.coord_to_data_index(coord).map(|index| unsafe {
            // SAFETY: `index` is guaranteed to be valid.
            self.data.get_unchecked(index)
        })
    }
}

impl<T> GridMut for VecGrid<T> {
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Self::Item> {
        self.coord_to_data_index(coord).map(|index| unsafe {
            // SAFETY: `index` is guaranteed to be valid.
            self.data.get_unchecked_mut(index)
        })
    }
}

index_impl!(VecGrid<T>, T);
index_mut_impl!(VecGrid<T>, T);
