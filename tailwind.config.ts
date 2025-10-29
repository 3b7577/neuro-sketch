import type { Config } from "tailwindcss";

export default {
	content: ["./src/**/*.{ts,tsx,mdx}"],
	theme: {
		extend: {
			colors: {
				bg: "hsl(var(--bg))",
				panel: "hsl(var(--panel))",
				surface: "hsl(var(--surface))",
				muted: "hsl(var(--muted))",
				border: "hsl(var(--border))",
				text: "hsl(var(--text))",
				"text-muted": "hsl(var(--text-muted))",
				accent: "hsl(var(--accent))",
				"accent-weak": "hsl(var(--accent-weak))",
				success: "hsl(var(--success))",
				warn: "hsl(var(--warn))",
				error: "hsl(var(--error))",
			},
			boxShadow: {
				card: "var(--elev-1)",
				float: "var(--elev-2)",
			},
			borderRadius: {
				sm: "var(--radius-sm)",
				md: "var(--radius-md)",
				lg: "var(--radius-lg)",
			},
			transitionDuration: {
				150: "150ms",
				200: "200ms",
			},
			fontSize: {
				title: ["20px", "1.3"],
				section: ["16px", "1.4"],
				body: ["14px", "1.5"],
				caption: ["12px", "1.4"],
			},
			fontFamily: {
				mono: ["ui-monospace", "SFMono-Regular", "Menlo", "Consolas", "Liberation Mono", "monospace"],
			},
		},
	},
	plugins: [],
} satisfies Config;
