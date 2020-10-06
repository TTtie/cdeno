import { openPlugin, ops } from "../mod.js";

openPlugin("./cdeno-test-lib.so");
const td = new TextDecoder;
const te = new TextEncoder;

const out = ops.test_op(te.encode("Hello there - this is a zero copy buffer from Deno!"), te.encode("more tests!"));
console.log(out);
const str = td.decode(out);

console.log(str);