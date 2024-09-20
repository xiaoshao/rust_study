#[derive(Copy, Clone)]
struct FooCopy {
    value: i16,
}

impl FooCopy {
    fn new() -> Self {
        Self {
            value: 0
        }
    }

    fn get(&self) -> i16 {
        self.value
    }

    fn increase(&mut self) {
        self.value += 1;
    }
}

fn is_FnMut<F: FnMut()> (_closure: &F) {
}

fn is_Copy<F: Copy> (_closure: &F) {
}
#[test]
pub fn test_move_fn() {

    let mut fn_copy = FooCopy::new();

    let mut c_with_move = move || {
        for _ in 0..5 {
            fn_copy.increase();
        }

        println!("foo_copy in closure with move {} ", fn_copy.get())
    };

    c_with_move();

    println!("foo_copy out of closure with move {}", fn_copy.get());

    let mut c_without_move = || {
        for _ in 0..5 {
            fn_copy.increase();
        }

        println!("foo_copy in closure(without move): {}", fn_copy.get());
    };

    is_FnMut(&c_with_move);
    is_Copy(&c_with_move);

    is_FnMut(&c_without_move);

    // c_without_move();

    // println!("foo_copy out of closure without move {}", fn_copy.get());
}