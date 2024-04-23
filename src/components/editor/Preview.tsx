import { useState, useEffect, useRef } from "react";
import { convertFileSrc } from '@tauri-apps/api/core'

import CONSTANTS from "../../constants";
import Spinner from "./Spinner";

const PREVIEW_WIDTH = 600;

export default function Preview({
	onPreviewLoad
}: Readonly<{
	onPreviewLoad: (e: number) => void
}>) {
	const previewFps = CONSTANTS.previewFps;
	const videoRef = useRef<HTMLVideoElement>(null) as React.MutableRefObject<HTMLVideoElement>;

	const [videoLoaded, setVideoLoaded] = useState(false);

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

		// Get video dimensions from query params
		const height = params.get("height");
		const width = params.get("width");

		const aspectRatio = Number(width) / Number(height);

		videoRef.current.width = PREVIEW_WIDTH;
		videoRef.current.height = PREVIEW_WIDTH / aspectRatio;

		// Set the source to the preview URL
		videoRef.current.src = previewUrl;
		videoRef.current.load();

		// Get duration (in seconds) after the video is loaded
		videoRef.current.addEventListener('loadedmetadata', () => {
			const duration = videoRef.current.duration;
			const frames = Math.floor(duration * previewFps);

			setVideoLoaded(true); // hide spinner
			onPreviewLoad(frames); // pass total frames upwards
		});

	}, []);

	return <div className="w-full h-fit rounded-xl overflow-hidden mt-5 mb-0 flex flex-col border border-neutral-600 items-center">
		{
			!videoLoaded && <Spinner />
		}
		<video
			className="object-cover"
			onClick={handlePlayPause}
			controls={false}
			ref={videoRef}
			style={{
				opacity: videoLoaded ? 1 : 0
			}}
			muted
		/>

	</div>
}