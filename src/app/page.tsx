"use client";
import { useNeuroSketchWasm } from "./hooks/useWasm";
import { useRef, useCallback, useEffect, useState } from "react";
import { renderWasmImage } from "@/lib/renderWasmImage";
import { GenerationMode } from "@/types";

const HEIGHT = 512;
const WIDTH = 1024;

const ColorLabel: Record<GenerationMode, string> = {
	[GenerationMode.GRAY_NOISE]: "gray noise",
	[GenerationMode.COLORED_NOISE]: "colored noise",
	[GenerationMode.PERLIN_NOISE]: "perlin noise",
	[GenerationMode.PALETTE]: "palette",
	[GenerationMode.HORIZONTAL_PALETTE]: "horizontal palette",
};

export default function Home() {
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const wasm = useNeuroSketchWasm();

	const [mode, setMode] = useState<GenerationMode>(GenerationMode.GRAY_NOISE);
	const [seed, setSeed] = useState<number>(0);

	const generateNewImage = useCallback(
		(customSeed?: number) => {
			if (!wasm || !wasm.exports || !canvasRef.current) return;

			if (customSeed !== undefined) {
				setSeed(customSeed);
			}

			const ctx = canvasRef.current.getContext("2d");
			if (!ctx) return;

			renderWasmImage(wasm.exports, ctx, mode, customSeed ?? seed, WIDTH, HEIGHT);
		},
		[wasm, mode, seed]
	);

	useEffect(() => {
		if (wasm.ready) {
			generateNewImage();
		}
	}, [wasm.ready, mode]);

	if (!wasm.ready) return "Loading...";

	return (
		<div className="min-h-screen w-full grid place-items-center p-4">
			<select
				className="mb-4 px-2 py-2 rounded border border-neutral-300 bg-neutral-50 dark:bg-neutral-900 dark:border-neutral-700 text-neutral-900 dark:text-neutral-100"
				value={mode}
				onChange={(e) => {
					setMode(Number(e.target.value));
				}}
			>
				{Object.entries(ColorLabel).map(([value, label]) => (
					<option key={value} value={value}>
						{label}
					</option>
				))}
			</select>

			<div className="flex items-center gap-2 mb-4">
				<label className="block text-sm font-medium text-neutral-700 dark:text-neutral-300">Seed:</label>
				<input
					type="number"
					min={0}
					step={1}
					value={seed}
					onChange={(e) => setSeed(Number(e.target.value))}
					className="px-2 py-1 rounded border border-neutral-300 dark:border-neutral-700 bg-neutral-50 dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100 w-32"
				/>
				<button
					type="button"
					className="px-3 py-1 rounded bg-neutral-200 dark:bg-neutral-700 hover:bg-neutral-300 dark:hover:bg-neutral-600 border border-neutral-300 dark:border-neutral-600 text-neutral-900 dark:text-neutral-100"
					onClick={() => generateNewImage(Math.floor(Math.random() * 1000000))}
				>
					Randomize
				</button>
			</div>

			<button
				className="px-4 py-2 bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-700 border border-neutral-300  rounded text-neutral-900 dark:text-neutral-100 cursor-pointer"
				onClick={() => generateNewImage()}
			>
				Generate new image
			</button>

			<canvas
				ref={canvasRef}
				width={WIDTH}
				height={HEIGHT}
				className="shadow-lg rounded-md border border-neutral-200 dark:border-neutral-800"
			/>
		</div>
	);
}
