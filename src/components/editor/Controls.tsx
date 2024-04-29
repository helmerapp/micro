import React from "react";
import CONSTANTS from "../../constants";
import ExportButton from "./ExportButton";

const Label = ({ text, children }: {
	text: string,
	children: React.ReactNode
}) => {
	return <label className="flex flex-col gap-2 w-full">
		<span className="text uppercase font-bold text-xs opacity-60">{text}</span>
		{children}
	</label>
}

const getEstimatedFileSize = (
	fps: number,
	width: number,
	height: number,
	quality: number,
	durationInFrames: number
) => {
	const durationInSeconds = durationInFrames / CONSTANTS.previewFps;
	const totalFrames = durationInSeconds * fps;
	const qFactor = quality / 100;

	const totalPixels = (width * height * totalFrames) / 3;

	const totalBytes = (totalPixels * qFactor + 1.5) / 2.5
	const totalKb = totalBytes / 1024;
	const totalMb = totalKb / 1024;
	return totalMb.toFixed(0);
}

export default function Controls({
	gifSettings,
	setGifSettings,
	exportHandler,
	selectedFrames,
	exporting
}: {
	gifSettings: {
		fps: number,
		size: number,
		speed: number,
		quality: number,
		loop_gif: boolean,
	},
	setGifSettings: (settings: any) => void,
	exportHandler: () => void,
	selectedFrames: number[],
	exporting: boolean
}) {

	const updateGifSettings = (key: string, val: any) => {
		setGifSettings({
			...gifSettings,
			[key]: val
		});
	}

	const { fps, size, speed, quality, loop_gif } = gifSettings;

	// TODO: implement quality setting in UI
	// Gifski quality setting: https://docs.rs/gifski/latest/gifski/struct.Settings.html#structfield.quality

	const durationInFrames = Math.abs(selectedFrames[1] - selectedFrames[0]);

	// TODO: pass aspect ratio from Rust
	const aspectRatio = 1 / 1;
	const width = size;
	const height = width * aspectRatio;

	const estimatedSize = getEstimatedFileSize(fps, width, height, quality, durationInFrames);

	return <form className="flex flex-col gap-6 h-fit w-full items-center">
		<div className="flex w-full justify-between gap-4">
			<Label text="Size">
				<select className="rounded-lg p-2 bg-black w-full"
					value={size}
					onChange={(e) => updateGifSettings("size", Number(e.target.value))}>
					<option value="200">200px</option>
					<option value="400">400px</option>
					<option value="800">800px</option>
					<option value="1000">1000px</option>
					<option value="1200">1200px</option>
				</select>
			</Label>
			<Label text="Smooth?">
				<select className="rounded-lg p-2 bg-black"
					value={fps}
					onChange={(e) => updateGifSettings("fps", Number(e.target.value))}>
					<option value="15">Meh</option>
					<option value="30">Good</option>
					<option value="60">Butter</option>
				</select>
			</Label>
			<Label text="Speed">
				<select className="rounded-lg p-2 bg-black"
					value={speed}
					onChange={(e) => updateGifSettings("speed", Number(e.target.value))}>
					<option value="0.5">Half</option>
					<option value="1">Normal</option>
					<option value="2">Double</option>
				</select>
			</Label>
			<Label text="Loop">
				<select className="rounded-lg p-2 bg-black"
					value={loop_gif ? "true" : "false"}
					onChange={(e) => e.target.value === "true"
						? updateGifSettings("loop_gif", true)
						: updateGifSettings("loop_gif", false)}>
					<option value="true">Yes</option>
					<option value="false">No</option>
				</select>
			</Label>
		</div>
		<p>Estimated GIF Size: {estimatedSize}mb </p>
		<ExportButton
			exporting={exporting}
			clickHandler={exportHandler}
		/>
	</form>
}