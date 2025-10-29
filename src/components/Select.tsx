import React from "react";

type Option<T extends string | number> = {
	label: string;
	value: T;
};

type SelectProps<T extends string | number> = {
	value: T;
	onChange: (value: T) => void;
	options: Option<T>[];
	label?: string;
	className?: string;
	name?: string;
	disabled?: boolean;
};

export function Select<T extends string | number>({
	value,
	onChange,
	options,
	label,
	className = "",
	name,
	disabled = false,
}: SelectProps<T>) {
	return (
		<label className="block">
			{label && <span className="block mb-1 text-caption text-text-muted">{label}</span>}
			<select
				className={`w-full input ${className}`}
				name={name}
				value={value as any}
				onChange={(e) => {
					const selected = options.find((opt) => String(opt.value) === e.target.value);
					if (selected) {
						onChange(selected.value);
					}
				}}
				disabled={disabled}
			>
				{options.map((option) => (
					<option key={option.value} value={option.value as any}>
						{option.label}
					</option>
				))}
			</select>
		</label>
	);
}
