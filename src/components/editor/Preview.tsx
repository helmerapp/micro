import { useState, useEffect, useRef } from "react";
import { convertFileSrc } from '@tauri-apps/api/tauri'
import * as Slider from '@radix-ui/react-slider';

export default function Preview({
	selectedFrames,
	setSelectedFrames
}: {
	selectedFrames: number[],
	setSelectedFrames: (e: number[]) => void
}) {
	const previewFps = 30; // Assume preview is 30 fps for now
	const videoRef = useRef<HTMLVideoElement>(null) as React.MutableRefObject<HTMLVideoElement>;
	const [totalFrames, setTotalFrames] = useState(0);


	const handleInput = (e: number[]) => {

		// check which handle is changing and update preview
		if (e[0] !== selectedFrames[0]) {
			videoRef.current.currentTime = e[0] / previewFps;
		} else if (e[1] !== selectedFrames[1]) {
			videoRef.current.currentTime = e[1] / previewFps;
		}

		setSelectedFrames(e)
	};

	const handlePlayPause = () => {
		if (videoRef.current.paused) {
			videoRef.current.play();
		} else {
			videoRef.current.pause();
		}
	}

	useEffect(() => {
		// Extract preview video path from query params
		const params = new URLSearchParams(window.location.search);
		const previewPath = params.get("file");
		const previewUrl = convertFileSrc(previewPath!);

		// Set the source to the preview URL
		videoRef.current.src = previewUrl;
		videoRef.current.load();

		// Get duration (in seconds) after the video is loaded
		videoRef.current.addEventListener('loadedmetadata', () => {
			const duration = videoRef.current.duration;
			const frames = Math.floor(duration * previewFps);
			setTotalFrames(frames);
			setSelectedFrames([0, frames]);
		});

	}, []);

	return <div className="w-[95%] h-fit rounded-xl overflow-hidden m-auto mt-0 mb-0 flex flex-col p-4 gap-4">
		<video muted controls={false}
			className="w-full object-cover rounded-md overflow-hidden h-[380px] border border-neutral-600" width={800} height={480} ref={videoRef}
			onClick={handlePlayPause}
		/>

		<Slider.Root
			min={0}
			step={1}
			max={totalFrames}
			value={selectedFrames}
			className="relative flex items-center select-none touch-none h-5"
			onValueChange={(e) => handleInput(e)}
		>
			<Slider.Track className="bg-black relative grow rounded-full h-[3px] w-full">
				<Slider.Range className="absolute bg-[aqua] rounded-full h-full" />
			</Slider.Track>
			<Slider.Thumb
				className="block w-5 h-5 bg-white shadow-[0_2px_10px] shadow-blackA4 rounded-[10px] hover:bg-violet3 focus:outline-none focus:shadow-[0_0_0_5px] focus:shadow-blackA5"
				aria-label="Start Frame"
			/>
			<Slider.Thumb
				className="block w-5 h-5 bg-white shadow-[0_2px_10px] shadow-blackA4 rounded-[10px] hover:bg-violet3 focus:outline-none focus:shadow-[0_0_0_5px] focus:shadow-blackA5"
				aria-label="End Frame"
			/>
		</Slider.Root>
	</div>
}