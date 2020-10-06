/*import { Plug } from "https://deno.land/x/plug@0.2.3/mod.ts";

const pluginPath = `${import.meta.url}/../target/release`;

const libcdeno = await Plug.prepare({
    name: "cdeno",
    url: `${pluginPath}/libcdeno.so`,
    policy: Plug.CachePolicy.NONE
});*/

// Although we have plug init above, CachePolicy.NONE still caches for some reason?
// Must be run relative to libcdeno.so
const libcdeno = Deno.openPlugin("./libcdeno.so");

const denoOps = Deno.core.ops();
const cdenoOpenPluginOp = denoOps["cdeno::open_plugin"];
const te = new TextEncoder;
const td = new TextDecoder;

function jsNumberToRustusize(num) {
    const intArray = Deno.build.arch === "x86_64" ? new BigUint64Array(1) : new Int32Array(1);
    intArray[0] = Deno.build.arch === "x86_64" ? num : Number(num);
    return new Uint8Array(intArray.buffer);
}

export function openPlugin(path) {
    const buf = te.encode(path);

    const out = Deno.core.dispatch(cdenoOpenPluginOp, buf);
    console.log(out);
    const outBuf = td.decode(out);
    console.log(outBuf);
    const opMap = JSON.parse(outBuf);
    for (const k in opMap) {
        if (!Object.prototype.hasOwnProperty.call(opMap, k)) continue;
        ops[k] = function (...args) {
            return Deno.core.dispatch(Number(opMap[k]), jsNumberToRustusize(BigInt(opMap[k])), ...args)
        }
    }
}

export const ops = {};

/**
 * Closes the cdeno plugin and destroys the module.
 * THIS REMOVES THE NATIVE PLUGIN, INCLUDING ALL PLUGINS LOADED USING CDENO.
 */
export function close() {
    Deno.close(libcdeno);
}