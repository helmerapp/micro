import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import Preview from "./Preview";
import Controls from "./Controls";
import Loader from './Loader';
import CONSTANTS from '../../constants';
import { listen } from '@tauri-apps/api/event';

const previewFps = CONSTANTS.previewFps;
export default function Editor() {

	const [selectedFrames, setSelectedFrames] = useState([0, 200]);
	const [exporting, setExporting] = useState(false);
	const [isPreviewLoading, setIsPreviewLoading] = useState(true);

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
				// We pass the range as time because the frame count here
				// does not match the frame count in rust
				range: [selectedFrames[0] / previewFps, selectedFrames[1] / previewFps],
				...options,
			}
		}).then(() => {
			console.log("export started")
		})
	}

	useEffect(() => {

		const previewListener = listen("preview-ready", () => {
			setIsPreviewLoading(false);
		});

		return () => {
			previewListener.then(unlisten => unlisten())
		}

	}, []);

	return (
		<main className="w-full h-full flex flex-col bg-[#222] p-8 items-center justify-center">
			{
				isPreviewLoading
					? <Loader />
					: <>
						<Preview setSelectedFrames={setSelectedFrames} selectedFrames={selectedFrames} />
						<Controls exportHandler={exportHandler} exporting={exporting} />
					</>
			}
		</main>
	);
}