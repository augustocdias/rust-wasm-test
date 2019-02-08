import * as wasm from "rust-wasm-test";

wasm.init();

let imageType;
let bytes;

function showImage(result) {
    let blob = new Blob([result], {"type": imageType});
    document.getElementById('targetImg').src = URL.createObjectURL(blob);
}


document.getElementById('uploader').addEventListener('change', function() {
    let reader = new FileReader();
        reader.onload = (ev) => {
            bytes = new Uint8Array(ev.target.result);
            let result = wasm.show_image(bytes, imageType.split('/')[1]);
            let blob1 = new Blob([bytes], {"type": imageType});
            let blob2 = new Blob([result], {"type": imageType});
			document.getElementById('sourceImg').src = URL.createObjectURL(blob1);
			document.getElementById('targetImg').src = URL.createObjectURL(blob2);
        };
    imageType = this.files[0].type;
    reader.readAsArrayBuffer(this.files[0]);
});

document.getElementById('greet').addEventListener('click', function() {
    wasm.greet();
});

document.getElementById('gray').addEventListener('click', function() {
    showImage(wasm.gray(bytes, imageType.split('/')[1]));
});

document.getElementById('fractal').addEventListener('click', function() {
    showImage(wasm.generate_fractal());
});

document.getElementById('flip').addEventListener('click', function() {
    showImage(wasm.flip(bytes, imageType.split('/')[1]));
});

document.getElementById('blur').addEventListener('click', function() {
    showImage(wasm.blur(bytes, imageType.split('/')[1], parseFloat(document.getElementById('blur_value').value)));
});

document.getElementById('bright').addEventListener('click', function() {
    showImage(wasm.brighten(bytes, imageType.split('/')[1], parseInt(document.getElementById('bright_value').value)));
});

document.getElementById('invert').addEventListener('click', function() {
    showImage(wasm.invert(bytes, imageType.split('/')[1]));
});