'use client';
import { useEffect, useState } from 'react';

type Wasm = typeof import('@/wasm/neuro_sketch_gen.js');

export function useNeuroSketchWasm() {
  const [exports, setExports] = useState<Wasm | null>(null);
  const [error, setError] = useState<unknown>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const mod = (await import('@/wasm/neuro_sketch_gen.js')) as Wasm;
        if (!cancelled) setExports(mod);
      } catch (e) {
        if (!cancelled) setError(e);
      }
    })();
    return () => { cancelled = true; };
  }, []);

  return { exports, ready: !!exports, error };
}