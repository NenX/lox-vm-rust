use lox_vm_rust::MyPeekable;

#[test]
fn test_peekable() {
    let mut iter = MyPeekable::new(1..5);
    assert_eq!(iter.peek(0), Some(&1));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.peek(0), Some(&2));
    assert_eq!(iter.peek(1), Some(&3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.peek(0), None);
    assert_eq!(iter.next(), None);
}

struct Ss {
    pub n: i32,
}
impl Ss {
    pub fn new(n: i32) -> Self {
        Self { n }
    }
    pub fn add(&mut self) {
        self.n += 1;
    }
}
fn call_add(mut ff: Box<dyn FnMut()>) {
    ff()
}
#[test]
fn test_peek_mut() {
    let mut s = Ss::new(1);
    s.add();
    let ff = move || s.add(); // 这里的s是可变的
    call_add(Box::new(ff));
}
