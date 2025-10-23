import { Wasm } from "@/types";

export const renderWasmImage = (
	wasm: Wasm,
	ctx: CanvasRenderingContext2D,
	mode: number,
	seed: number,
	w: number,
	h: number
) => {
	const srcArray = wasm.generate(mode, seed, w, h);
	const buf = new ArrayBuffer(srcArray.byteLength);
	const data = new Uint8ClampedArray(buf);
	data.set(srcArray);

	const imageData = new ImageData(data, w, h);

	ctx.putImageData(imageData, 0, 0);

	return imageData;
};
