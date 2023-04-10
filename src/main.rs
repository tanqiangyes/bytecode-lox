mod chunk;

use chunk::*;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);
    chunk.disassemble("test");
    chunk.free();
}
