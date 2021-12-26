#[derive(Debug)]
struct CPU {
    current_operation: u16,
    register: [u8; 2],
}

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        register: [0; 2],
    };

    cpu.current_operation = 0x8014;
    cpu.register[0] = 5;
    cpu.register[1] = 10;

    println!("{:?}", cpu);
}
