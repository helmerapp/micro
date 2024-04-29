import { useState, useEffect, useRef } from "react";
import * as Slider from '@radix-ui/react-slider';
import CONSTANTS from "../../constants";

const { previewFps } = CONSTANTS;

export default function Trimmer({
	totalFrames,
	selectedFrames,
	setSelectedFrames,
	videoRef
}: {
	totalFrames: number
	selectedFrames: number[],
	setSelectedFrames: (e: number[]) => void,
	videoRef: React.MutableRefObject<HTMLVideoElement>
}) {

	const [audioContext, setAudioContext] = useState<null | AudioContext>(null);
	const [currentFrame, setCurrentFrame] = useState(0);

	const animationRef = useRef<any>(null);

	const videoEl = videoRef.current;


	useEffect(() => {
		const updateFrame = () => {

			// If video is paused, don't update frame
			if (videoEl.paused) {
				animationRef.current = requestAnimationFrame(updateFrame);
				return;
			}

			// If video is playing, get current frame from video timestamp
			const currentFrame = Math.floor(videoEl.currentTime * previewFps);

			if (currentFrame >= selectedFrames[1]) {
				// Current frame is beyond the last frame selected
				// TODO: fetch loop details
				const loop = true;
				if (loop) {
					videoEl.currentTime = selectedFrames[0] / previewFps;
				} else {
					videoEl.pause();
				}// 
			} else if (currentFrame < selectedFrames[0]) {
				// Current frame is before the first frame selected
				videoEl.currentTime = selectedFrames[0] / previewFps;
			} else {
				// Current frame is within the selected range
				setCurrentFrame(currentFrame)
			}

			// Call updateFrame recursively
			animationRef.current = requestAnimationFrame(updateFrame);
		};

		// Call updateFrame the first time
		animationRef.current = requestAnimationFrame(updateFrame);

		return () => {
			cancelAnimationFrame(animationRef.current);
		};
	}, [videoRef, selectedFrames]);


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
				videoEl.pause();

				if (e[0] !== selectedFrames[0]) {
					videoEl.currentTime = e[0] / previewFps;
					setCurrentFrame(e[0])
				} else if (e[1] !== selectedFrames[1]) {
					videoEl.currentTime = e[1] / previewFps;
					setCurrentFrame(e[1])
				}

				playClickSound();
				setSelectedFrames(e)
			}}
		>
			<Slider.Track className="relative w-full">
				<div className="w-full h-4 flex justify-between items-end">
					{Array.from({ length: totalFrames }, (_, i) => {

						let color = "bg-[rgba(255,255,255,0.3)]"

						// if i is included in selected frames
						if (i >= selectedFrames[0] && i <= selectedFrames[1]) {
							color = "bg-[orange]"
						}

						if (i === currentFrame) {
							return <div key={i} className={`w-full h-full bg-[white] ml-[1px] mr-[1px]`} />
						}

						if (i % previewFps === selectedFrames[0]) {
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