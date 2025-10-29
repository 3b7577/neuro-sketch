"use client";

import { useEffect, useRef } from "react";
import { useNeuroSketchWasm } from "@/app/hooks/useWasm";
import { renderWasmImage } from "@/lib/renderWasmImage";
import useSettingsStore from "@/store/useSettingsStore";

const CANVAS_WIDTH = 1024;
const CANVAS_HEIGHT = 768;

const RatingCanvas = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { exports, ready, error } = useNeuroSketchWasm();
  const { mode, seed } = useSettingsStore();

  useEffect(() => {
    if (!ready || !exports || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");

    if (!ctx) return;

    try {
      renderWasmImage(exports, ctx, mode, seed, CANVAS_WIDTH, CANVAS_HEIGHT);
    } catch (err) {
      console.error("Error rendering image:", err);
    }
  }, [ready, exports, mode, seed]);

  if (error) {
    return (
      <div className="flex items-center justify-center p-8 text-error">
        <p>Failed to load WASM module</p>
      </div>
    );
  }

  return (
    <div className="flex items-center justify-center p-6">
      <canvas
        ref={canvasRef}
        width={CANVAS_WIDTH}
        height={CANVAS_HEIGHT}
        className="border border-border rounded-md bg-surface shadow-card"
      />
    </div>
  );
};

export default RatingCanvas;
