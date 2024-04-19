import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core'

import CONSTANTS from '../../constants';
import Preview from "./Preview";
import Controls from "./Controls";
import Trimmer from './Trimmer';

const previewFps = CONSTANTS.previewFps;

export default function Editor() {
	const [exporting, setExporting] = useState(false);
	const [totalFrames, setTotalFrames] = useState(0);
	const [selectedFrames, setSelectedFrames] = useState([0, totalFrames]);

	const exportHandler = (options: {
		fps: number,
		size: number,
		speed: number,
		bounce: boolean,
		loop_gif: boolean
	}) => {
		setExporting(true);
		invoke('export_handler', {
			options: {
				// Pass the range as time because it may be
				// different than frame count in rust
				range: [selectedFrames[0] / previewFps, selectedFrames[1] / previewFps],
				...options,
			}
		}).then(() => {
			console.log("export started")
		})
	}

	return (
		<main className="w-full h-full flex flex-col bg-[#222] p-8 items-center">
			<Preview
				selectedFrames={selectedFrames}
				onPreviewLoad={(f) => setTotalFrames(f)}
			/>
			<Trimmer
				totalFrames={totalFrames}
				selectedFrames={selectedFrames}
				setSelectedFrames={setSelectedFrames}
			/>
			<Controls
				exportHandler={exportHandler}
				selectedFrames={selectedFrames}
				exporting={exporting}
			/>
		</main>
	);
}