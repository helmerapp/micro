import { useState, useEffect, useRef } from "react";
import { convertFileSrc } from '@tauri-apps/api/core'
import { motion } from "framer-motion";

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
		<div className="w-full h-full mt-2 mb-0 ">
			<motion.video
				className="object-contain max-w-full w-auto h-auto rounded-xl overflow-hidden m-auto p-0 perspective-[800px] max-h-[660px]"
				animate={{
					scale: [0.2, 1],
					y: ["-100%", "0%"],
					rotateZ: [30, 0],
					skew: [30, 0],
					filter: ["blur(100px)", "blur(0px)"],
					borderRadius: ["50%", "0.75rem"],
					boxShadow: ["0px 0px 0px 0px rgba(0,0,0,0)", "0px 12px 24px 0px rgba(0,0,0,0.2)"],
				}}
				transition={{ duration: 2, ease: "anticipate" }}
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