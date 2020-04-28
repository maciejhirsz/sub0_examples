
struct Foo {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

impl Foo {
    fn cross_add(&mut self, index: usize) -> Option<()> {
        let a = self.get_mut_1(index)?;
        let b = self.get_mut_2(index)?;

        *a += *b;
        *b += *a;

        Some(())
    }

    fn get_mut_1(&mut self, index: usize) -> Option<&mut i32> {
        self.vec1.get_mut(index)
    }

    fn get_mut_2(&mut self, index: usize) -> Option<&mut i32> {
        self.vec2.get_mut(index)
    }
}
