# cdeno
*Bringing C/C++ closer to Deno* - A FFI interface for Deno plugins

## Caveats
- **This is only a proof of concept and there may be a performance penalty.** If you care about performance, I recommend linking your library to a Rust plugin directly.
- This is very fragile and involves returning void pointers around. Returning or passing an invalid pointer to cdeno will 100% likely crash the whole deno binary.
- Requires the plugin/library to be both loadable by system's dynamic linker and by Deno. 
- Requires a Deno native plugin
- Tested only on my GNU/Linux-based machine. Not tested on either Windows or macOS.


## Usage
See [the example](example/)