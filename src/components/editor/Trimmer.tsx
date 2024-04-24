import { useState, useEffect } from "react";
import * as Slider from '@radix-ui/react-slider';
import CONSTANTS from "../../constants";

export default function Trimmer({
	totalFrames,
	selectedFrames,
	setSelectedFrames,
	// speed = 1,
	// fps = 30,
}: {
	totalFrames: number
	selectedFrames: number[],
	setSelectedFrames: (e: number[]) => void,
	// speed: number,
	// fps: number
}) {

	const previewFps = CONSTANTS.previewFps;
	const [audioContext, setAudioContext] = useState<null | AudioContext>(null);

	useEffect(() => {
		const initializeAudioContext = () => {
			// @ts-expect-error: webkitAudioContext is not in the lib but needed for iOS / Safari
			const context = new (window.AudioContext || window.webkitAudioContext)();
			setAudioContext(context);
		};

		initializeAudioContext();
	}, []);

	const playClickSound = () => {
		if (!audioContext) return;

		const buffer = audioContext.createBuffer(1, audioContext.sampleRate * 0.02, audioContext.sampleRate);
		const data = buffer.getChannelData(0);

		for (let i = 0; i < buffer.length; i++) {
			data[i] = Math.random() * 2 - 1; // Generate random noise
		}

		const source = audioContext.createBufferSource();
		source.buffer = buffer;

		const filter = audioContext.createBiquadFilter();
		filter.type = 'bandpass';
		filter.frequency.value = 5000;

		const gainNode = audioContext.createGain();
		gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);

		source.connect(filter);
		filter.connect(gainNode);
		gainNode.connect(audioContext.destination);

		source.start();
		gainNode.gain.exponentialRampToValueAtTime(0.00001, audioContext.currentTime + 0.02);
		source.stop(audioContext.currentTime + 0.02);
	};

	return <div className="w-full h-fit flex flex-col gap-2 mt-5 pb-4 mb-1">
		<Slider.Root
			min={0}
			step={1}
			max={totalFrames}
			value={selectedFrames}
			className="relative items-center select-none touch-none h-fit"
			onValueChange={(e: number[]) => {

				// TODO: this should in Preview.tsx but 🤷‍♂️
				const videoEl = document.querySelector("video") as HTMLVideoElement;

				if (e[0] !== selectedFrames[0]) {
					videoEl.currentTime = e[0] / previewFps;
				} else if (e[1] !== selectedFrames[1]) {
					videoEl.currentTime = e[1] / previewFps;
				}

				playClickSound();
				setSelectedFrames(e)
			}}
		>
			<Slider.Track className="relative w-full">
				<div className="w-full h-6 flex justify-between items-end">
					{Array.from({ length: totalFrames }, (_, i) => {

						let color = "bg-[rgba(255,255,255,0.3)]"

						// if i is included in selected frames
						if (i >= selectedFrames[0] && i <= selectedFrames[1]) {
							color = "bg-[orange]"
						}

						if (i % previewFps === 0) {
							return (
								<div key={i} className={`w-full h-full ${color} ml-[1px] mr-[1px]`} />
							)
						} else {
							return (
								<div key={i} className={`w-full h-2/3 ${color} ml-[1px] mr-[1px] rounded-lg`} />
							)
						}
					})}
				</div>
				{/* <Slider.Range className="absolute bg-white rounded-full h-full" /> */}
			</Slider.Track>
			<Slider.Thumb
				className="block w-2 h-3 bg-[orange] rounded-full translate-y-1"
				aria-label="Start Frame"
			/>
			<Slider.Thumb
				className="block w-2 h-3 bg-[orange] rounded-full translate-y-1"
				aria-label="End Frame"
			/>
		</Slider.Root>
	</div>
}