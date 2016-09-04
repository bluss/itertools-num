
quickcheck! {
    fn size_linspace(a: f32, b: f32, n: usize) -> bool {
        let it = itertools::linspace(a, b, n);
        it.len() == n &&
            exact_size(it)
    }
}

