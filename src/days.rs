macro_rules! impl_days {
    ($($m:ident,)*) => {
        pub fn days() -> Vec<usize> {
            vec![$($m::DAY,)*]
        }

        pub fn solver(day: usize) -> Option<fn(&str) -> DaySolution> {
            match day {
                $($m::DAY => Some(|input: &str|{
                    let (part_1, part_2) = $m::solve(&input);
                    (part_1.map(|p| p.into()), part_2.map(|p| p.into()))
                }),)*
                _ => None,
            }
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/days.in"));

type DaySolution = (Option<PartSolution>, Option<PartSolution>);

macro_rules! part_solution {
    ($($i:ident => $t:ty,)*) => {
        pub enum PartSolution {
            $($i($t),)*
        }

        impl std::fmt::Display for PartSolution {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                match self {
                    $(PartSolution::$i(x) => x.fmt(f),)*
                }
            }
        }

        $(
            impl From<$t> for PartSolution {
                fn from(x: $t) -> Self {
                    PartSolution::$i(x)
                }
            }
        )*
    };
}

part_solution! {
    I8 => i8,
    I16 => i16,
    I32 => i32,
    I64 => i64,
    I128 => i128,
    Isize => isize,
    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,
    U128 => u128,
    Usize => usize,
    String => String,
}
