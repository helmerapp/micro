import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core'
import { PostHogProvider, usePostHog } from "posthog-js/react"

import CONSTANTS from '../../constants';
import Preview from "./Preview";
import Controls from "./Controls";
import Trimmer from './Trimmer';

const previewFps = CONSTANTS.previewFps;
const posthogKey = import.meta.env.PUBLIC_POSTHOG_KEY;
const posthogHost = import.meta.env.PUBLIC_POSTHOG_HOST;

export default function Editor() {
	const [exporting, setExporting] = useState(false);
	const [totalFrames, setTotalFrames] = useState(0);
	const [selectedFrames, setSelectedFrames] = useState([0, totalFrames]);
	const [isOkSharingUsageData, setIsOkSharingUsageData] = useState(true);

	useEffect(() => {
		setSelectedFrames([0, totalFrames]);
	}, [totalFrames]);

	useEffect(() => {
		invoke('is_ok_sharing_usage_data').then((res) => {
			console.log("Is Ok Sharing Usage Data", res);
			setIsOkSharingUsageData(res as boolean);
		})
	}, [])

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

		if (isOkSharingUsageData) {
			posthog.capture('gif_exported', {
				fps: options.fps,
				size: options.size,
				speed: options.speed,
				loop: options.loop_gif,
				duration: Math.abs(selectedFrames[1] - selectedFrames[0]) / CONSTANTS.previewFps,
			});
		}

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
		<PostHogProvider
			apiKey={posthogKey}
			options={{
				api_host: posthogHost,
				api_transport: "fetch"
			}}
		>
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
		</PostHogProvider>
	);
}