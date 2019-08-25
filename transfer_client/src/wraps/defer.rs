pub struct Defer<F: FnMut()>(Option<F>);

impl<F: FnMut()> std::ops::Drop for Defer<F> {
	fn drop(&mut self) {
        self.0.take().map(|mut f| f());
	}
}

pub fn defer<F>(f: F) -> impl std::ops::Drop
    where F: FnMut() {
    Defer(Some(f))
}

#[test]
fn dropTest() {
    use std::collections::HashMap;
    let mut values = HashMap::new();
    {
        let key = "1";
        values.insert(key, "test");
        assert_eq!(values.len(), 1);
        defer(|| {
            values.remove(key);
        })
    }
    assert_eq!(values.len(), 0);
}
