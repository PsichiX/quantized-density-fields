use std::fmt::Debug;

pub trait State: Sized + Clone + Default + Debug + Subdividable {}

impl State for () {}
impl State for bool {}
impl State for i8 {}
impl State for i16 {}
impl State for i32 {}
impl State for i64 {}
impl State for u8 {}
impl State for u16 {}
impl State for u32 {}
impl State for u64 {}
impl State for f32 {}
impl State for f64 {}
impl State for isize {}
impl State for usize {}

pub trait Subdividable {
    fn subdivide(&self, subdivisions: usize) -> Self;
}

impl Subdividable for () {
    fn subdivide(&self, _: usize) -> Self { *self }
}
impl Subdividable for bool {
    fn subdivide(&self, _: usize) -> Self { *self }
}
impl Subdividable for i8 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for i16 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for i32 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for i64 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for u8 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for u16 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for u32 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for u64 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for f32 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for f64 {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for isize {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
impl Subdividable for usize {
    fn subdivide(&self, subdivisions: usize) -> Self { self / subdivisions as Self }
}
