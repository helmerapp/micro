import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core'
import { usePostHog } from "posthog-js/react"

import CONSTANTS from '../../constants';
import Controls from "./Controls";
import Posthog from '../Posthog';
import Preview from "./Preview";

const { previewFps } = CONSTANTS;

export default function Editor() {
	const posthog = usePostHog();

	const [exporting, setExporting] = useState(false);
	const [selectedFrames, setSelectedFrames] = useState([0, 0]);
	const [gifSettings, setGifSettings] = useState({
		fps: 30,
		speed: 1,
		size: 1000,
		quality: 90,
		loop_gif: true,
	});

	const exportHandler = () => {

		// Prevent multiple exports
		if (exporting) return;
		setExporting(true);

		const options = {
			// Pass the range as time because it may be
			// different than frame count in rust
			...gifSettings,
			range: [selectedFrames[0] / previewFps, selectedFrames[1] / previewFps],
		}

		posthog?.capture('GifExported', {
			'FPS': options.fps,
			'Size': options.size,
			'Speed': options.speed,
			'Loop': options.loop_gif,
			'Quality': options.quality,
			'Duration': Math.abs(selectedFrames[1] - selectedFrames[0]) / CONSTANTS.previewFps,
		});

		invoke('export_gif', { options }).then(() => setExporting(false));
	}

	return (
		<Posthog>
			<main className="w-full h-full flex flex-col bg-[#181818] p-8 items-center">
				<Preview
					selectedFrames={selectedFrames}
					setSelectedFrames={setSelectedFrames}
				/>
				<Controls
					exporting={exporting}
					gifSettings={gifSettings}
					selectedFrames={selectedFrames}
					exportHandler={exportHandler}
					setGifSettings={setGifSettings}
				/>
			</main>
		</Posthog>
	);
}