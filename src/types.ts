export type Wasm = typeof import("@/wasm/neuro_sketch_gen.js");

export enum GenerationMode {
	GRAY_NOISE = 0,
	COLORED_NOISE = 1,
	PERLIN_NOISE = 2,
	PALETTE = 3,
}
