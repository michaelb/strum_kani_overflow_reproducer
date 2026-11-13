use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter, Debug)]
enum Request {
    Bark,
    Bite,
}

fn main() {
    for variant in Request::iter(){
        println!("{variant:?}");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn nth_overflow() {
        let mut it = Request::iter();
        let should_be_none = it.nth(usize::MAX); // panics from overflow
        assert!(should_be_none.is_none());

        let mut it = Request::iter();
        let _ = it.next();
        let should_be_none = it.nth(usize::MAX - 1 ); // also panics from overflow
        assert!(should_be_none.is_none());

        let mut it = Request::iter();
        it.next_back();         // back_idx = 1
        let should_be_none = it.nth(usize::MAX - 1); // also panics
        assert!(should_be_none.is_none());
    }
}
