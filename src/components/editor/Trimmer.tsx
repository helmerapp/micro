import { useState, useEffect, useRef } from "react";
import { motion } from "framer-motion";
import * as Slider from '@radix-ui/react-slider';
import CONSTANTS from "../../constants";

const { previewFps } = CONSTANTS;

export default function Trimmer({
	videoRef,
	totalFrames,
	selectedFrames,
	setSelectedFrames,
}: {
	videoRef: React.MutableRefObject<HTMLVideoElement>
	totalFrames: number
	selectedFrames: number[],
	setSelectedFrames: (e: number[]) => void,
}) {

	const [audioContext, setAudioContext] = useState<null | AudioContext>(null);
	const [currentFrame, setCurrentFrame] = useState(0);

	const animationRef = useRef<any>(null);
	const videoEl = videoRef.current;

	useEffect(() => {
		setSelectedFrames([0, totalFrames]);
	}, [totalFrames])


	useEffect(() => {
		const updateFrame = () => {

			// If video is paused, don't update frame
			if (videoEl?.paused) {
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

	return <div className="w-full h-fit flex flex-col gap-2 mt-8 pb-4 mb-1">
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
			<Slider.Track className="relative w-full flex items-center justify-center">
				<div className="w-[99%] h-4 flex justify-between items-end">
					{Array.from({ length: totalFrames }, (_, i) => {

						const selected = selectedFrames[0] <= i && i <= selectedFrames[1];
						const current = i === currentFrame;
						const seconds = (i - selectedFrames[0]) / previewFps;

						return <Frame key={i} selected={selected} current={current} seconds={seconds} />
					})}
				</div>
			</Slider.Track>
			<Slider.Thumb
				className="inline-block w-2 h-2 bg-[orange] rounded-full translate-y-[-6px]"
				aria-label="Start Frame"
			/>
			<Slider.Thumb
				className="inline-block w-2 h-2 bg-[orange] rounded-full translate-y-[-6px]"
				aria-label="End Frame"
			/>
		</Slider.Root>
	</div>
}


const Frame = ({ selected, current, seconds }: {
	selected: boolean,
	current: boolean,
	seconds: number
}) => {
	return <motion.div className={`relative w-[1px] rounded`}
		style={{
			backgroundColor: current
				? "white"
				: selected
					? "orange"
					: "rgba(255,255,255,0.3)",
			height: (seconds % 1 === 0) || current
				? "100%"
				: selected
					? "50%"
					: "4px",
			transformOrigin: "bottom bottom",
		}}
	>
		{
			seconds % 1 === 0
				? <div className="absolute text-xs text-white bottom-5 text-center opacity-40 translate-x-[-50%]">{seconds}s</div>
				: null
		}
	</motion.div>

}