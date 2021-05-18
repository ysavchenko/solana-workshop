const BufferLayout = require("buffer-layout");

const INSTRUCTION = BufferLayout.union(BufferLayout.u8("instruction"));
INSTRUCTION.addVariant(0, undefined, "initialize");

function encodeInstructionData(instruction) {
  let b = Buffer.alloc(instructionMaxSpan);
  let span = INSTRUCTION.encode(instruction, b);
  return b.slice(0, span);
}

module.exports = {
  encodeInstructionData,
}