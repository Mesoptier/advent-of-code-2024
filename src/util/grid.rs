use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub type Coord = (usize, usize);

pub trait Grid<'a> {
    type Item;

    fn width(&'a self) -> usize;
    fn height(&'a self) -> usize;
    fn get(&'a self, coord: Coord) -> Option<Self::Item>;

    fn iter(&'a self) -> impl Iterator<Item = (Coord, Self::Item)> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (x, y)))
            .map(move |coord| (coord, self.get(coord).unwrap()))
    }

    fn map<B, F>(self, f: F) -> MapGrid<'a, Self, F>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B,
    {
        MapGrid::new(self, f)
    }
}

pub trait RefGrid<'a>: Grid<'a, Item = &'a Self::RefItem> + 'a {
    type RefItem;
}

pub trait RefGridMut<'a>: RefGrid<'a> {
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Self::RefItem>;
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

impl<'a> Grid<'a> for StrGrid<'a> {
    type Item = &'a u8;

    fn width(&self) -> usize {
        self.line_width - 1
    }

    fn height(&self) -> usize {
        self.data.len() / self.line_width
    }

    fn get(&self, coord: Coord) -> Option<&'a u8> {
        self.coord_to_data_index(coord).map(|index| unsafe {
            // SAFETY: `index` is guaranteed to be valid.
            self.data.get_unchecked(index)
        })
    }
}

impl<'a> RefGrid<'a> for StrGrid<'a> {
    type RefItem = u8;
}

impl<'a> Index<Coord> for StrGrid<'a> {
    type Output = <Self as RefGrid<'a>>::RefItem;

    fn index(&self, index: Coord) -> &Self::Output {
        self.get(index).unwrap()
    }
}

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

impl<'a, T: 'a> Grid<'a> for VecGrid<T> {
    type Item = &'a T;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn get(&'a self, coord: Coord) -> Option<Self::Item> {
        self.coord_to_data_index(coord).map(|index| unsafe {
            // SAFETY: `index` is guaranteed to be valid.
            self.data.get_unchecked(index)
        })
    }
}

impl<'a, T: 'a> RefGrid<'a> for VecGrid<T> {
    type RefItem = T;
}

impl<'a, T: 'a> RefGridMut<'a> for VecGrid<T> {
    fn get_mut(&mut self, coord: Coord) -> Option<&mut Self::RefItem> {
        self.coord_to_data_index(coord).map(|index| unsafe {
            // SAFETY: `index` is guaranteed to be valid.
            self.data.get_unchecked_mut(index)
        })
    }
}

impl<T> Index<Coord> for VecGrid<T> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<Coord> for VecGrid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

pub struct MapGrid<'g, G, F> {
    grid: G,
    f: F,
    _marker: PhantomData<&'g G>,
}

impl<G, F> MapGrid<'_, G, F> {
    pub fn new(grid: G, f: F) -> Self {
        Self {
            grid,
            f,
            _marker: PhantomData,
        }
    }
}

impl<'a, B, G, F> Grid<'a> for MapGrid<'a, G, F>
where
    G: Grid<'a>,
    F: Fn(G::Item) -> B,
{
    type Item = B;

    fn width(&'a self) -> usize {
        self.grid.width()
    }

    fn height(&'a self) -> usize {
        self.grid.height()
    }

    fn get(&'a self, coord: Coord) -> Option<Self::Item> {
        self.grid.get(coord).map(&self.f)
    }
}
