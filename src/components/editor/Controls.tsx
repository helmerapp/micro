import React, { useState } from "react";
import * as Switch from '@radix-ui/react-switch';
import CONSTANTS from "../../constants";

const Label = ({ text, children }: {
	text: string,
	children: React.ReactNode
}) => {
	return <label className="flex flex-col gap-2">
		<span className="text">{text}</span>
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

	const totalPixels = width * height * totalFrames;
	const totalBytes = totalPixels * 4 * qFactor;
	const totalKb = totalBytes / 1024;
	const totalMb = totalKb / 1024;
	return totalMb;
}

export default function Controls({
	exportHandler,
	selectedFrames,
}: {
	exportHandler: (data: {
		fps: number,
		size: number,
		speed: number,
		bounce: boolean,
		loop_gif: boolean
	}) => void,
	selectedFrames: number[],
	exporting: boolean
}) {

	const [fps, setFps] = useState(30);
	const [size, setSize] = useState(1000);
	const [loop, setLoop] = useState(false);
	const [speed, setSpeed] = useState(1);
	const [bounce, setBounce] = useState(false);

	const totalFrames = Math.abs(selectedFrames[1] - selectedFrames[0]);

	const estimatedSize = getEstimatedFileSize(fps, totalFrames, size, size, 100);

	return <form className="flex flex-col p-6 gap-8 rounded-lg">
		<div className="flex align-middle justify-center w-fit  gap-8 ">
			<Label text="Size">
				<select className="rounded-lg p-2 bg-black"
					defaultValue={"1000"}
					onChange={(e) => setSize(Number.parseFloat(e.target.value))}>
					<option value="200">200px</option>
					<option value="400">400px</option>
					<option value="800">800px</option>
					<option value="1000">1000px</option>
					<option value="1200">1200px</option>
					{/* <option value="2000">2000px</option> */}
				</select>
			</Label>
			<Label text="Smoothness">
				<input type="range" min="15" max="60" value={fps}
					onChange={e => setFps(Number(e.target.value))}
				/>
			</Label>
			<Label text="Speed">
				<input type="range" min="0.5" max="2"
					value={speed} step="0.1"
					onChange={e => setSpeed(Number(e.target.value))} />
			</Label>
			<Label text="Loop">
				<Switch.Root
					className="w-[42px] h-[25px] bg-[#111] rounded-full relative focus:shadow-black data-[state=checked]:bg-black outline-none cursor-default"
					id="loop"
					checked={loop}
					onCheckedChange={e => setLoop(e)}
				>
					<Switch.Thumb className="block w-[21px] h-[21px] bg-white rounded-full transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
				</Switch.Root>
			</Label>
			{/* TODO: add bounce later */}
			{/* <Label text="Bounce">
				<Switch.Root
					className="w-[42px] h-[25px] bg-[#111] rounded-full relative focus:shadow-black data-[state=checked]:bg-black outline-none cursor-default"
					id="bounce"
					checked={bounce}
					onCheckedChange={e => setBounce(e)}
				>
					<Switch.Thumb className="block w-[21px] h-[21px] bg-white rounded-full transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
				</Switch.Root>
			</Label> */}
		</div>
		<p>Estimated GIF Size: {estimatedSize}mb </p>
		<input type="submit" value="Export to Desktop"
			className="bg-[#444] text-white rounded-lg p-2 hover:scale-105 transition-all cursor-pointer"
			onClick={e => {
				e.preventDefault();
				exportHandler({ size, fps, speed, loop_gif: loop, bounce })
			}}
		/>
	</form>


}