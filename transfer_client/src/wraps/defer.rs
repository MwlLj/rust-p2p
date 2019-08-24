struct Defer<F: FnMut()>(Option<F>);

impl<F: FnMut()> Drop for Defer {
	fn drop(&mut self) {
		match self.0 {
			Some(f) => {
				f();
			},
			None => {
			}
		}
	}
}
