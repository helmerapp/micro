import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { motion } from "framer-motion";
import { usePostHog } from "posthog-js/react";

const MAX_RECORDING_LIMIT_SECONDS = 20;

const RecordButton = () => {
	const posthog = usePostHog();
	const [area, setArea] = useState(false);
	const [recording, setRecording] = useState(false);

	useEffect(() => {
		listen("reset-area", () => setArea(false));
		listen("updated-crop-area", () => setArea(true));

		document.addEventListener("keydown", handleKeyDown);

		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, []);

	const handleKeyDown = (e: KeyboardEvent) => {
		if (e.key === "Escape" || e.key === "Esc") {
			e.preventDefault();
			invoke("hide_cropper").then(() => stopRecording());
		}
	};

	const startRecording = () => {
		invoke("start_recording");
		posthog?.capture("RecordingStarted");
		setRecording(true);
	};

	const stopRecording = () => {
		if (!recording) return;
		setRecording(false);
		invoke("stop_recording");
	};

	if (!area) {
		return <p className="text-xs font-semibold tracking-tight w-full text-center">Drag to select an area</p>
	} else {
		return (
			<motion.main
				className="m-auto w-fit h-fit p-1 rounded-full overflow-hidden relative"
				animate={{
					backgroundImage: recording
						? `conic-gradient(white 360deg, lightseagreen 360deg, lightseagreen 360deg)`
						: `conic-gradient(white 0deg, lightseagreen 0deg, lightseagreen 360deg)`
				}}
				transition={recording ? {
					duration: MAX_RECORDING_LIMIT_SECONDS,
					ease: "linear"
				} : undefined}
				onAnimationComplete={(e) => {
					// TODO: refactor this to use framer motion variants
					// @ts-expect-error: backgroundImage is not a valid property
					if (e['backgroundImage'] === `conic-gradient(white 360deg, lightseagreen 360deg, lightseagreen 360deg)`) {
						stopRecording();
					}
				}}
			>
				<button
					className="w-8 h-8 flex items-center justify-center rounded-full outline-none overflow-hidden cursor-pointer bg-[lightseagreen] transition-transform"
					onClick={() => recording ? stopRecording() : startRecording()}
				>
					{
						recording
							? (
								<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round">
									<rect x="2" y="2" width="20" height="20"></rect>
								</svg>
							)
							: (
								<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round">
									<circle cx="12" cy="12" r="10"></circle>
								</svg>
							)
					}
				</button>
			</motion.main>
		);
	}
};

export default RecordButton;
