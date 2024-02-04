import React, { useState } from "react";

const Label = ({ text, children }: {
	text: string,
	children: React.ReactNode
}) => {
	return <label className="flex flex-col items-center gap-2">
		<span>{text}</span>
		{children}
	</label>
}

export default function Controls({
	submitHandler: any
}) {

	const [size, setSize] = useState(1000);
	const [fps, setFps] = useState(30);
	const [speed, setSpeed] = useState(1);
	const [loop, setLoop] = useState(true);
	const [bounce, setBounce] = useState(true);

	return <form id="form"
		className="flex bg-[#111] p-6 rounded-lg gap-4 align-middle justify-center w-fit">
		<Label text="Size">
			<select name="size" id="size" onChange={(e) => {
				setSize(Number.parseFloat(e.target.value))
			}}>
				<option value="200">200</option>
				<option value="400">400</option>
				<option value="800">800</option>
				<option value="1000" selected>1000</option>
				<option value="2000">2000</option>
			</select>
		</Label>
		<Label text="Smoothness">
			<input id="fps" type="range" min="15" max="60" defaultValue="30"
			// onChange={e => setFps(e.target.value)}
			/>
		</Label>
		<Label text="Speed">
			<input id="speed" type="range" min="0.5" max="2" defaultValue="1" step="0.1" />
		</Label>
		<Label text="Loop">
			<input id="loop" type="checkbox" />
		</Label>
		<Label text="Bounce">
			<input id="bounce" type="checkbox" />
		</Label>
		<input type="submit" value="Export"
			className="bg-[#444] text-white rounded-lg p-2"
			onClick={e => {
				e.preventDefault();
				submitHandler({
					size,
					fps,
					speed,
					loop,
					bounce
				})
			}}
		/>
	</form>


}