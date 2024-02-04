import { useState } from "react";
import * as Slider from '@radix-ui/react-slider';

export default function Preview() {

	const [selectedFrames, setSelectedFrames] = useState([0, 200]);

	const handleInput = (e) => {
		console.log(e)
		setSelectedFrames(e)
	};

	// TODO: Show the GIF preview here
	// get preview video file
	// get total frames and map to slider max
	// on slider change, update start and end frame
	// on slider change, update preview video to show current frame

	return <div className="bg-[#111111] w-[80%] h-fit rounded-xl overflow-hidden m-auto mt-0 mb-10 flex flex-col p-4 gap-4">
		<img src="http://placekitten.com/1000/600" alt="" className="w-full" />

		<Slider.Root
			className="relative flex items-center select-none touch-none h-5"
			value={selectedFrames}
			min={0}
			max={200}
			step={1}
			onValueChange={(e) => handleInput(e)}
		>
			<Slider.Track className="bg-blackA7 relative grow rounded-full h-[3px] w-full">
				<Slider.Range className="absolute bg-[yellow] rounded-full h-full" />
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