import Link from "next/link";

export default function Home() {
  return (
    <main className="min-h-screen bg-bg text-text">
      <section className="max-w-5xl mx-auto px-6 py-16 flex flex-col items-center gap-6">
        <h1 className="text-title font-bold text-center">NeuroSketch</h1>
        <p className="text-section text-center text-text-muted max-w-3xl">
          Deterministic, high-fidelity generative art—powered by WebAssembly and
          noise-based algorithms. Drive the visuals with a single seed for
          perfectly repeatable results.
        </p>

        <div className="flex items-center gap-3">
          <Link href="/rating" className="btn-primary">
            Try the Generator
          </Link>
        </div>
      </section>

      <section className="max-w-5xl mx-auto px-6 pb-4">
        <div className="card">
          <h2 className="text-title font-bold mb-3">What is NeuroSketch?</h2>
          <p className="text-body text-text-muted">
            NeuroSketch is a procedural image generator that blends multiple
            noise functions and curated palettes. It emphasizes clarity,
            accessibility, and repeatability: every image can be reproduced by
            reusing its seed and parameters. Built with Next.js, TypeScript,
            Tailwind, and a Rust WASM core for performance.
          </p>
        </div>
      </section>

      <section className="max-w-5xl mx-auto px-6 py-8 grid gap-4 md:grid-cols-2">
        <div className="card">
          <h3 className="text-section font-semibold mb-2">
            Deterministic by design
          </h3>
          <p className="text-body text-text-muted">
            Every output is derived from a numeric seed—share the seed,
            reproduce the art exactly.
          </p>
        </div>
        <div className="card">
          <h3 className="text-section font-semibold mb-2">
            Multiple generation modes
          </h3>
          <p className="text-body text-text-muted">
            Noise, Colored Noise, Perlin, and Palette exploration give you
            distinct aesthetics.
          </p>
        </div>
        <div className="card">
          <h3 className="text-section font-semibold mb-2">
            WebAssembly performance
          </h3>
          <p className="text-body text-text-muted">
            A Rust core compiles to WASM for fast, interactive generation right
            in the browser.
          </p>
        </div>
        <div className="card">
          <h3 className="text-section font-semibold mb-2">Export and share</h3>
          <p className="text-body text-text-muted">
            Export crisp PNGs and snapshot parameters so others can recreate
            your results.
          </p>
        </div>
      </section>
    </main>
  );
}
