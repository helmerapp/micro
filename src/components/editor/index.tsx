import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import Preview from "./Preview";
import Controls from "./Controls";

export default function Editor() {

	const [selectedFrames, setSelectedFrames] = useState([0, 200]);
	const [exporting, setExporting] = useState(false);

	const exportHandler = (options: {
		size: number,
		fps: number,
		speed: number,
		loop_gif: boolean,
		bounce: boolean
	}) => {
		setExporting(true);
		invoke('export_handler', {
			options: {
				range: selectedFrames,
				...options,
			}
		}).then(() => {
			console.log("export started")
		})
	}

	return (
		<main className="w-full h-full flex flex-col bg-[#222] p-8 items-center">
			<Preview setSelectedFrames={setSelectedFrames} selectedFrames={selectedFrames} />
			<Controls exportHandler={exportHandler} exporting={exporting} />
		</main>
	);
}