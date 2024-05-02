import { useState, useEffect, useRef } from "react";
import { convertFileSrc } from '@tauri-apps/api/core'

import Spinner from "./Spinner";
import Trimmer from "./Trimmer";
import CONSTANTS from "../../constants";

const PREVIEW_WIDTH = 600;

const { previewFps } = CONSTANTS;

export default function Preview({
	selectedFrames,
	setSelectedFrames,
}: {
	selectedFrames: number[],
	setSelectedFrames: React.Dispatch<React.SetStateAction<number[]>>,
}) {

	const [totalFrames, setTotalFrames] = useState(0);
	const videoRef = useRef<HTMLVideoElement>(null) as React.MutableRefObject<HTMLVideoElement>;

	const handlePlayPause = () => videoRef.current.paused
		? videoRef.current.play()
		: videoRef.current.pause();

	useEffect(() => {
		// Extract preview video path from query params
		const params = new URLSearchParams(window.location.search);
		const previewPath = params.get("file");
		const previewUrl = convertFileSrc(previewPath!);

		// Get video dimensions and compute aspect ratio
		const height = Number(params.get("height"));
		const width = Number(params.get("width"));
		const aspectRatio = width / height;

		videoRef.current.width = PREVIEW_WIDTH;
		videoRef.current.height = PREVIEW_WIDTH / aspectRatio;

		// Set the source to the preview URL
		videoRef.current.src = previewUrl;
		videoRef.current.load();

		const calculateTotalFrames = () => {
			const duration = videoRef.current?.duration;
			const frames = Math.floor(duration * previewFps);
			setTotalFrames(frames);
		};

		videoRef.current?.addEventListener('loadedmetadata', () => {
			calculateTotalFrames();
		});
	}, []);


	return <>
		<div className="w-full h-fit rounded-xl overflow-hidden mt-2 mb-0 bg-neutral-900 ">
			{totalFrames === 0 && <Spinner />}
			<video
				style={{ opacity: totalFrames > 0 ? 1 : 0 }}
				className="object-contain max-w-full w-fit h-auto rounded-xl overflow-hidden"
				onClick={handlePlayPause}
				controls={false}
				ref={videoRef}
				muted
			/>
		</div>
		<Trimmer
			videoRef={videoRef}
			totalFrames={totalFrames}
			selectedFrames={selectedFrames}
			setSelectedFrames={setSelectedFrames}
		/>
	</>
}