use core::arch::asm;

fn add_1() {
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }

    println!("in (3) + 5 = {}", o);
}

fn add_2(x: u64, y: u64) {
    let out: u64;
    unsafe {
        asm!(
            "mov {0}, {2}",
            "add {0}, {1}",
            out(reg) out,
            in(reg) y,
            in(reg) x,
        )
    }

    println!("{x} + {y} = {out}");
}

fn main() {
    add_1();

    add_2(5, 10);
}
