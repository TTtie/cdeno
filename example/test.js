import { openPlugin, ops } from "../mod.js";

openPlugin("./cdeno-test-lib.so");
const td = new TextDecoder;
const te = new TextEncoder;
console.log("Calling sync test op");
const out = ops.test_op(te.encode("Hello there - this is a zero copy buffer from Deno!"), te.encode("more tests!"));
const str = td.decode(out);

console.log(str);

setTimeout(() => {
    console.log("[JS] This shouldn't block!");
}, 2500);


Deno.core.setAsyncHandler(ops.test_op_async.id, resp => {
    const str = td.decode(resp);
    console.log("Got from async op!");
    console.log(str);
});
console.log("Calling async test op");
ops.test_op_async(te.encode("Hello there - this is a async zero copy buffer from Deno!"), te.encode("more tests!"));