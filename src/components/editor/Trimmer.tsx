import { useState, useEffect } from "react";
import * as Slider from '@radix-ui/react-slider';
import CONSTANTS from "../../constants";

export default function Trimmer({
	totalFrames,
	selectedFrames,
	setSelectedFrames
}: {
	totalFrames: number
	selectedFrames: number[],
	setSelectedFrames: (e: number[]) => void,
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

	return <>
		<div className="w-full h-4 flex justify-between items-end">
			{Array.from({ length: totalFrames }, (_, i) => {

				let color = "bg-[rgba(255,255,255,0.3)]"

				// if i is included in selected frames
				if (i >= selectedFrames[0] && i <= selectedFrames[1]) {
					color = "bg-[orange]"
				}

				if (i % previewFps === 0) {
					return (
						<div key={i} className={`w-[2px] h-full ${color}`} />
					)
				} else {
					return (
						<div key={i} className={`w-[2px] h-1/2 ${color}`} />
					)
				}
			})}
		</div>

		<Slider.Root
			min={0}
			step={1}
			max={totalFrames}
			value={selectedFrames}
			className="relative flex items-center select-none touch-none h-5"
			onValueChange={(e: number[]) => {
				playClickSound();
				setSelectedFrames(e)
			}}
		>
			<Slider.Track className="bg-[transparent] relative grow rounded-full h-[3px] w-full">
				<Slider.Range className="absolute bg-[transparent] rounded-full h-full" />
			</Slider.Track>
			<Slider.Thumb
				className="block w-5 h-6 bg-[orange] rounded-2xl rounded-ss-3xl rounded-se-3xl translate-x-[-10px]"
				aria-label="Start Frame"
			/>
			<Slider.Thumb
				className="block w-5 h-6 bg-[orange] rounded-2xl rounded-ss-3xl rounded-se-3xl translate-x-[10px]"
				aria-label="End Frame"
			/>
		</Slider.Root>
	</>
}