"use client";
import { useNeuroSketchWasm } from "./hooks/useWasm";
import { useRef, useCallback, useEffect } from "react";
import { renderWasmImage } from "@/lib/renderWasmImage";

const HEIGHT = 512;
const WIDTH = 1024;

export default function Home() {
	const canvasRef = useRef<HTMLCanvasElement>(null);
	const wasm = useNeuroSketchWasm();

	const generateNewImage = useCallback(() => {
		if (!wasm || !wasm.exports || !canvasRef.current) return;

		const ctx = canvasRef.current.getContext("2d");
		if (!ctx) return;

		return renderWasmImage(wasm.exports, ctx, 1, Math.random() * 10000, WIDTH, HEIGHT);
	}, [wasm]);

	useEffect(() => {
		if (wasm.ready) {
			generateNewImage();
		}
	}, [wasm.ready]);

	if (!wasm.ready) return "Loading...";

	return (
		<div className="min-h-screen w-full grid place-items-center p-4">
			<canvas
				ref={canvasRef}
				width={WIDTH}
				height={HEIGHT}
				className="shadow-lg rounded-md border border-neutral-200 dark:border-neutral-800"
			/>

			<button
				className="px-4 py-2 bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-700 border border-neutral-300  rounded text-neutral-900 dark:text-neutral-100 cursor-pointer"
				onClick={generateNewImage}
			>
				Generate new image
			</button>
		</div>
	);
}
