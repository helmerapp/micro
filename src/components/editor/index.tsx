import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core'
import Preview from "./Preview";
import Controls from "./Controls";
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
				range: [selectedFrames[0]/previewFps, selectedFrames[1]/previewFps],
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
		<main className="w-full h-full flex flex-col bg-[#222] p-8 items-center">
			{
				isPreviewLoading ? 
				<span id="loader"><style>
					{`
						#loader {
							width: 12px;
							height: 12px;
							border-radius: 50%;
							display: block;
							margin:15px auto;
							position: relative;
							color: #FFF;
							box-sizing: border-box;
							animation: animloader 2s linear infinite;
						}
						
						@keyframes animloader {
							0% {
								box-shadow: 14px 0 0 -2px,  38px 0 0 -2px,  -14px 0 0 -2px,  -38px 0 0 -2px;
							}
							25% {
								box-shadow: 14px 0 0 -2px,  38px 0 0 -2px,  -14px 0 0 -2px,  -38px 0 0 2px;
							}
							50% {
								box-shadow: 14px 0 0 -2px,  38px 0 0 -2px,  -14px 0 0 2px,  -38px 0 0 -2px;
							}
							75% {
								box-shadow: 14px 0 0 2px,  38px 0 0 -2px,  -14px 0 0 -2px,  -38px 0 0 -2px;
							}
							100% {
								box-shadow: 14px 0 0 -2px,  38px 0 0 2px,  -14px 0 0 -2px,  -38px 0 0 -2px;
							}
						}
					`}
				</style></span>
				:
				<>
					<Preview setSelectedFrames={setSelectedFrames} selectedFrames={selectedFrames} />
					<Controls exportHandler={exportHandler} exporting={exporting} />
				</>
			}
		</main>
	);
}