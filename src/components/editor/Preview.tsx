import { useState, useEffect, useRef } from "react";
import { convertFileSrc } from '@tauri-apps/api/tauri'
import * as Slider from '@radix-ui/react-slider';

export default function Preview() {

	const [previewVideo, setPreviewVideo] = useState<string>();
	const [selectedFrames, setSelectedFrames] = useState([0, 200]);

	const previewRef = useRef<HTMLVideoElement>(null);

	const handleInput = (e: number[]) => {
		console.log(e)
		setSelectedFrames(e)
	};

	// TODO: Show the GIF preview here

	useEffect(() => {
		// We have got the preview video file from query parameters
		const params = new URLSearchParams(window.location.search);
		const filePath = params.get("file");
		const assetUrl = convertFileSrc(filePath!);
		setPreviewVideo(assetUrl);
	}, []);

	useEffect(() => {
		if (previewVideo) {
			previewRef.current?.load()
			console.log(previewRef.current?.duration);
		}

	}, [previewVideo])

	// get total frames and map to slider max
	// on slider change, update preview video to show current frame

	return <div className="w-[95%] h-fit rounded-xl overflow-hidden m-auto mt-0 mb-0 flex flex-col p-4 gap-4">
		<video src={previewVideo} muted controls={false}
			className="w-full object-cover rounded-md overflow-hidden h-[380px] border border-neutral-600" width={800} height={480} ref={previewRef}
		/>

		<Slider.Root
			className="relative flex items-center select-none touch-none h-5"
			value={selectedFrames}
			min={0}
			max={200}
			step={1}
			onValueChange={(e) => handleInput(e)}
		>
			<Slider.Track className="bg-black relative grow rounded-full h-[3px] w-full">
				<Slider.Range className="absolute bg-[aqua] rounded-full h-full" />
			</Slider.Track>
			<Slider.Thumb
				className="block w-5 h-5 bg-white shadow-[0_2px_10px] shadow-blackA4 rounded-[10px] hover:bg-violet3 focus:outline-none focus:shadow-[0_0_0_5px] focus:shadow-blackA5"
				aria-label="Start Frame"
			/>
			<Slider.Thumb
				className="block w-5 h-5 bg-white shadow-[0_2px_10px] shadow-blackA4 rounded-[10px] hover:bg-violet3 focus:outline-none focus:shadow-[0_0_0_5px] focus:shadow-blackA5"
				aria-label="End Frame"
			/>
		</Slider.Root>
	</div>
}