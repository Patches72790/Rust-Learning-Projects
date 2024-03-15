use core::arch::asm;

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
    add_2(5, 10);

    add_2(25, 30);
}
