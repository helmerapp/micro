import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core'
import { usePostHog } from "posthog-js/react"

import CONSTANTS from '../../constants';
import Controls from "./Controls";
import Posthog from '../Posthog';
import Preview from "./Preview";
import Trimmer from './Trimmer';

const { previewFps } = CONSTANTS;

export default function Editor() {
	const posthog = usePostHog();

	const [exporting, setExporting] = useState(false);
	const [totalFrames, setTotalFrames] = useState(0);
	const [selectedFrames, setSelectedFrames] = useState([0, totalFrames]);
	const [gifSettings, setGifSettings] = useState({
		fps: 30,
		size: 1000,
		speed: 1,
		quality: 90,
		loop_gif: true,
	});

	useEffect(() => {
		setSelectedFrames([0, totalFrames]);
	}, [totalFrames]);

	const exportHandler = () => {

		if (exporting) return;

		setExporting(true);

		const options = {
			// Pass the range as time because it may be
			// different than frame count in rust
			...gifSettings,
			range: [selectedFrames[0] / previewFps, selectedFrames[1] / previewFps],
		}

		// TODO: add app version to the event
		posthog?.capture('GifExported', {
			'FPS': options.fps,
			'Size': options.size,
			'Speed': options.speed,
			'Loop': options.loop_gif,
			'Duration': Math.abs(selectedFrames[1] - selectedFrames[0]) / CONSTANTS.previewFps,
		});

		invoke('export_gif', {
			options
		}).then(() => {
			setExporting(false);
		})
	}

	return (
		<Posthog>
			<main className="w-full h-full flex flex-col bg-[#181818] p-8 items-center">
				<Preview
					onPreviewLoad={(f) => setTotalFrames(f)}
				/>
				<Trimmer
					totalFrames={totalFrames}
					selectedFrames={selectedFrames}
					setSelectedFrames={setSelectedFrames}
				/>
				<Controls
					exporting={exporting}
					gifSettings={gifSettings}
					setGifSettings={setGifSettings}
					exportHandler={exportHandler}
					selectedFrames={selectedFrames}
				/>
			</main>
		</Posthog>
	);
}