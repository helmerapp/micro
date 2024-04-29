import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getAll, getCurrent } from "@tauri-apps/api/window";
import { motion } from "framer-motion";
import { usePostHog } from "posthog-js/react";

const MAX_RECORDING_LIMIT_SECONDS = 20;

const RecordButton = () => {
	const [recording, setRecording] = useState(false);
	const posthog = usePostHog();

	useEffect(() => {
		document.addEventListener("keydown", handleKeyDown);
		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, []);

	const handleKeyDown = (event: KeyboardEvent) => {
		// TODO: just escape? or other keys too?
		if (event.key === "Escape" || event.key === "Esc") {
			const cropperWindow = getAll().find((win) => win.label === "cropper");
			const recordButtonWindow = getCurrent();
			cropperWindow?.hide();
			cropperWindow?.emit("reset-cropper");
			recordButtonWindow?.hide();
			stopRecording();
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
				className="w-12 h-12 flex items-center justify-center rounded-full outline-none overflow-hidden cursor-pointer bg-[lightseagreen] transition-transform"
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
};

export default RecordButton;
