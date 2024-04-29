import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getAll } from "@tauri-apps/api/window";
import { motion } from "framer-motion";
import "./toolbar.css";

import { usePostHog } from "posthog-js/react";
import Posthog from "../Posthog";

const MAX_RECORDING_LIMIT_SECONDS = 20;

const ToolbarReact = () => {
	const [recording, setRecording] = useState(false);
	const posthog = usePostHog();

	useEffect(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.key === "Escape" || event.key === "Esc") {

				const cropperWindow = getAll().find((win) => win.label === "cropper");
				cropperWindow?.hide();
				cropperWindow?.emit("reset-cropper");

				invoke("hide_toolbar");
				stopRecording();
			}
		};
		document.addEventListener("keydown", handleKeyDown);
		return () => {
			document.removeEventListener("keydown", handleKeyDown);
		};
	}, []);

	const startRecording = () => {
		invoke("start_recording");
		posthog?.capture("RecordingStarted");
		setRecording(true);
	};

	const stopRecording = () => {
		if (!recording) return;
		setRecording(false);
		posthog?.capture("RecordingStopped");
		invoke("stop_recording", {});
	};

	const handleClick = () => recording ? stopRecording() : startRecording();

	return (
		<Posthog>
			<motion.main
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
				<button className="record" onClick={handleClick}>
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
		</Posthog>
	);
};

export default ToolbarReact;
