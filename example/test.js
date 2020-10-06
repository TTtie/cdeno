import { openPlugin, ops } from "../mod.js";

openPlugin("./cdeno-test-lib.so");
const td = new TextDecoder;

const out = ops.test_op();
console.log(out);
const str = td.decode(out);

console.log(str);