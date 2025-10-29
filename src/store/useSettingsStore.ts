import { GenerationMode } from "@/types";
import { create } from "zustand";

interface ISettingsStoreValues {
	mode: GenerationMode;
	seed: number;
}

interface ISettingsStore extends ISettingsStoreValues {
	setMode: (mode: GenerationMode) => void;
	setSeed: (seed: number) => void;
}

const settingsStoreValues: ISettingsStoreValues = {
	mode: GenerationMode.GRAY_NOISE,
	seed: 0,
};

const useSettingsStore = create<ISettingsStore>((set) => ({
	...settingsStoreValues,
	setMode: (mode) => set({ mode }),
	setSeed: (seed) => set({ seed }),
}));

export default useSettingsStore;
