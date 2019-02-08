# A Simple Web Assembly Image Processing Test 

This is a simple test to process images using web assembly. It was made using Rust and the the library [image](https://github.com/PistonDevelopers/image)

## Building 

### Requirements

* [rustup](https://rustup.rs)
* [wasm-pack](https://github.com/rustwasm/wasm-pack)

### Building and Running

1. Run `wasm-pack` on the root of the project. This will generate the `pkg` directory, which contains the bindings from `rust` to `javascript`.
2. Go to `www` folder and run `npm install`
3. Go to `pkg` folder and run `npm link`
4. Go to `www` folder and run `npm link rust-wasm-test`
5. Inside `www` folder, run `npm run start`
6. Navigate to http://localhost:8080

I've mapped it to support only `BMP`, `PNG`, and `GIF`(without animation) formats. I've tried `JPEG`, but it seems the underlying library from `image` called [jpeg-decoder](https://github.com/kaksmet/jpeg-decoder) does not work in WASM.