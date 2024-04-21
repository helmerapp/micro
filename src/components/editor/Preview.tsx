import { useState, useEffect, useRef } from "react";
import { convertFileSrc } from '@tauri-apps/api/core'

import CONSTANTS from "../../constants";
import Spinner from "./Spinner";

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

	return <div className="w-[95%] h-fit rounded-xl overflow-hidden m-auto mt-0 mb-0 flex flex-col p-4 gap-2">
		{
			!videoLoaded && <Spinner />
		}
		<video
			className="w-full object-cover rounded-md overflow-hidden h-[380px] border border-neutral-600"
			onClick={handlePlayPause}
			controls={false}
			ref={videoRef}
			height={480}
			width={800}
			style={{
				opacity: videoLoaded ? 1 : 0
			}}
			muted
		/>

	</div>
}