use gottis::instructions::events::Instruction;

fn main() {
    let data = [2, 244, 96, 5, 0];
    let instruction = Instruction::from_bytes(&data);
    dbg!(instruction);
}
