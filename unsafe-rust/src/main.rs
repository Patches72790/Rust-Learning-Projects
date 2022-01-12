use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[repr(C)]
union Thing {
    first: i32,
    second: u8,
    third: i128,
}

impl std::fmt::Display for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                Thing { first } => write!(f, "{}", first),
                Thing { second } => write!(f, "{}", second),
                Thing { third } => write!(f, "{}", third),
            }
        }
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = split_at_mut(&mut nums[..], 2);

    println!("{:?} :: {:?}", a, b);

    unsafe {
        println!("Abs value of {} is {}!", -2, abs(-2));
    }

    let t = Thing { third: 12345678910 };

    println!(
        "Size of Thing: {} Bytes : {}",
        std::mem::size_of::<Thing>(),
        t
    );

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("Hi from a closure!"));
    f();

    let v = 0u32..20;
    let vr = v.collect::<Vec<u32>>();

    let my_closure = returns_closure();
    my_closure(42);
}

fn returns_closure() -> Box<(dyn Fn(i32) + 'static)> {
    Box::new(|x| println!("Hi from a returned closure and a num {}!", x))
}
