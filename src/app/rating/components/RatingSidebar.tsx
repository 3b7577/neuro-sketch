"use client";

import { FC, ReactNode, useState, useEffect } from "react";
import Sidebar from "@/components/Sidebar";
import { Select } from "@/components/Select";
import { GenerationMode } from "@/types";
import useSettingsStore from "@/store/useSettingsStore";

interface RatingSidebarProps {
  children?: ReactNode;
  className?: string;
}

const MODE_OPTIONS = [
  { label: "Gray Noise", value: GenerationMode.GRAY_NOISE },
  { label: "Colored Noise", value: GenerationMode.COLORED_NOISE },
  { label: "Perlin Noise", value: GenerationMode.PERLIN_NOISE },
  { label: "Palette", value: GenerationMode.PALETTE },
];

const randomSeed = (): number => {
  return Math.floor(Math.random() * 2147483647); // Max 32-bit int
};

export { randomSeed };

const RatingSidebar: FC<RatingSidebarProps> = ({
  children,
  className = "",
}) => {
  const { mode, seed, setMode, setSeed } = useSettingsStore();
  const [seedInput, setSeedInput] = useState<string>(seed.toString());

  useEffect(() => {
    setSeedInput(seed.toString());
  }, [seed]);

  const handleSeedChange = (value: string) => {
    setSeedInput(value);
    const numValue = parseInt(value, 10);
    if (!isNaN(numValue)) {
      setSeed(numValue);
    }
  };

  const handleRandomizeSeed = () => {
    const newSeed = randomSeed();
    setSeed(newSeed);
    setSeedInput(newSeed.toString());
  };

  return (
    <Sidebar className={`w-[272px] ${className}`}>
      <div className="flex flex-col gap-6">
        <div className="flex flex-col gap-4">
          <Select
            label="Mode"
            value={mode}
            onChange={setMode}
            options={MODE_OPTIONS}
          />

          <div className="flex flex-col gap-2">
            <label className="block">
              <span className="block mb-1 text-caption text-text-muted">
                Seed
              </span>
              <div className="flex gap-2">
                <input
                  type="number"
                  value={seedInput}
                  onChange={(e) => handleSeedChange(e.target.value)}
                  className="flex-1 input"
                  placeholder="Enter seed"
                />
                <button
                  onClick={handleRandomizeSeed}
                  className="btn-primary"
                  type="button"
                >
                  Random
                </button>
              </div>
            </label>
          </div>
        </div>

        {children}
      </div>
    </Sidebar>
  );
};

export default RatingSidebar;
