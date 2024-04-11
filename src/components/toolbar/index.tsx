import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./toolbar.css";
import useInterval from "../../utils/useInterval";

const RECORDING_LIMIT = 20;

const ToolbarReact = () => {
	const [seconds, setSeconds] = useState(RECORDING_LIMIT);
	const [recording, setRecording] = useState(false);

	// TODO: remove the use of useInterval
	useInterval(() => {
		if (recording && seconds >= 0) {
			seconds === 0
				? stopRecording()
				: setSeconds((prevSeconds) => prevSeconds - 1);
		}
	}, 1000);

	useEffect(() => {
		const mainEl = document.querySelector("main");
		if (mainEl) {
			const step = 360 / RECORDING_LIMIT;
			const deg = (seconds * step);
			mainEl.style.backgroundImage = `conic-gradient(lightseagreen ${deg}deg, white ${deg}deg, white 360deg)`
		}
	}, [seconds])

	useEffect(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.key === "Escape" || event.key === "Esc") {
				// TODO implement hide cropper window!
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
		invoke("start_capture");
		setRecording(true);
	};

	const stopRecording = () => {
		setRecording(false);
		setSeconds(RECORDING_LIMIT);
		invoke("stop_capture", {});
	};

	const handleClick = () => recording ? stopRecording() : startRecording();

	return (
		<main>
			<button className="record" onClick={handleClick}>
				{recording ? (
					<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round">
						<rect x="2" y="2" width="20" height="20"></rect>
					</svg>
				) : (
					<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round">
						<circle cx="12" cy="12" r="10"></circle>
					</svg>
				)}
			</button>
		</main>
	);
};

export default ToolbarReact;
