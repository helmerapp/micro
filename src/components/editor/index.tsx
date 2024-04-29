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
	const [exporting, setExporting] = useState(false);
	const [totalFrames, setTotalFrames] = useState(0);
	const [selectedFrames, setSelectedFrames] = useState([0, totalFrames]);

	const posthog = usePostHog();

	useEffect(() => {
		setSelectedFrames([0, totalFrames]);
	}, [totalFrames]);

	const exportHandler = (options: {
		fps: number,
		size: number,
		speed: number,
		bounce: boolean,
		loop_gif: boolean
	}) => {
		setExporting(true);

		posthog?.capture('GifExportStarted', {
			fps: options.fps,
			size: options.size,
			speed: options.speed,
			loop: options.loop_gif,
			duration: Math.abs(selectedFrames[1] - selectedFrames[0]) / CONSTANTS.previewFps,
		});

		invoke('export_handler', {
			options: {
				// Pass the range as time because it may be
				// different than frame count in rust
				range: [selectedFrames[0] / previewFps, selectedFrames[1] / previewFps],
				...options,
			}
		}).then(() => {
			console.log("Export Finished")
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
					exportHandler={exportHandler}
					selectedFrames={selectedFrames}
				/>
			</main>
		</Posthog>
	);
}