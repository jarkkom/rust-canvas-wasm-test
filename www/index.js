'use strict';

import { CanvasRenderer } from 'canvas-wasm-test';
import { memory } from 'canvas-wasm-test/canvas_wasm_test_bg';

const renderer = CanvasRenderer.new();
const w = renderer.width();
const h = renderer.height();

const canvas = document.getElementById('canvas');
canvas.width = w;
canvas.height = h;

const ctx = canvas.getContext('2d');

let frame = 0;

const loop = () => {
    // renderer.setCameraPosition(
    //     Math.cos(2 * Math.PI * frame / 512) * 250,
    //     150 + Math.sin(2 * Math.PI * frame / 512),
    //     Math.sin(2 * Math.PI * frame / 512) * 250);
    // renderer.setCameraTarget(0, 100, 0);
    renderer.setCameraPosition(
        Math.cos(2 * Math.PI * frame / 512) * 2.5,
        1.50 + Math.sin(2 * Math.PI * frame / 512),
        Math.sin(2 * Math.PI * frame / 300) * 2.50);
    renderer.setCameraTarget(0, 0, 0);
    renderer.render();
    frame++;

    const bufferPtr = renderer.buffer();
    const buffer = new Uint8ClampedArray(memory.buffer, bufferPtr, w * h * 4);
    const imageData = new ImageData(buffer, w, h);
    ctx.putImageData(imageData, 0, 0);
    requestAnimationFrame(loop);
}

const cubeObject = fetch('models/cube.obj').then((resp) => resp.text());
const catsObject = fetch('models/cats.obj').then((resp) => resp.text());
const torusObject = fetch('models/torus.obj').then((resp) => resp.text());
const teapotObject = fetch('models/teapot.obj').then((resp) => resp.text());
const stoneObject = fetch('models/stone.obj').then((resp) => resp.text());

const rustTexture = new Promise((resolve, reject) => {
    const img = new Image();
    img.src = 'models/envmap2.jpg';

    img.onload = () => {
        const tmp = document.createElement('canvas');
        tmp.width = img.naturalWidth;
        tmp.height = img.naturalHeight;

        const ctx = tmp.getContext('2d');
        ctx.drawImage(img, 0, 0);

        resolve(ctx.getImageData(0, 0, tmp.width, tmp.height));
    };
});

const objs = [rustTexture, torusObject, cubeObject];

Promise.all(objs).then((values) => {
    renderer.add_obj(values[1], values[0].data, values[0].width, values[0].height);
    //renderer.add_obj(values[2], values[0].data, values[0].width, values[0].height);
    loop();
}).catch((reason) => {
    console.log(reason);
});
