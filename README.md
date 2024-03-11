# Building

Compile using:

### `wasm-pack build --target web`

# Installing

Rename resulting 'pkg' directory into 'md-rust'. Copy the directory in the root folder of the React app and install it using:

### `npm i ./md-rust`

# Usage

The package exports a default async function which must be called first in order to load the wasm:

### `import init, { function_1, function_2 } from 'md-rust';`
