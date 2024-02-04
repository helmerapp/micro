import React, { useState } from "react";
import * as Switch from '@radix-ui/react-switch';

const Label = ({ text, children }: {
	text: string,
	children: React.ReactNode
}) => {
	return <label className="flex flex-col gap-2">
		<span className="text">{text}</span>
		{children}
	</label>
}

export default function Controls({ submitHandler }: {
	submitHandler: (data: { size: number, fps: number, speed: number, loop_gif: boolean, bounce: boolean }) => void
}) {

	const [size, setSize] = useState(1000);
	const [fps, setFps] = useState(30);
	const [speed, setSpeed] = useState(1);
	const [loop, setLoop] = useState(false);
	const [bounce, setBounce] = useState(false);

	return <form className="flex bg-[#111] p-6 rounded-lg gap-8 align-middle justify-center w-fit">
		<Label text="Size">
			<select name="size"
				defaultValue={"1000"}
				onChange={(e) => {
					setSize(Number.parseFloat(e.target.value))
				}}>
				<option value="200">200</option>
				<option value="400">400</option>
				<option value="800">800</option>
				<option value="1000">1000</option>
				<option value="2000">2000</option>
			</select>
		</Label>
		<Label text="Smoothness">
			<input type="range" min="15" max="60" defaultValue="30"
				onChange={e => setFps(Number(e.target.value))}
			/>
		</Label>
		<Label text="Speed">
			<input type="range" min="0.5" max="2" defaultValue="1" step="0.1"
				onChange={e => setSpeed(Number(e.target.value))} />
		</Label>
		<Label text="Loop">
			<Switch.Root
				className="w-[42px] h-[25px] bg-blackA6 rounded-full relative shadow-[0_2px_10px] shadow-blackA4 focus:shadow-[0_0_0_2px] focus:shadow-black data-[state=checked]:bg-black outline-none cursor-default"
				id="loop"
				checked={loop}
				onCheckedChange={e => setLoop(e)}
			>
				<Switch.Thumb className="block w-[21px] h-[21px] bg-white rounded-full shadow-[0_2px_2px] shadow-blackA4 transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
			</Switch.Root>
		</Label>
		<Label text="Bounce">
			<Switch.Root
				className="w-[42px] h-[25px] bg-blackA6 rounded-full relative shadow-[0_2px_10px] shadow-blackA4 focus:shadow-[0_0_0_2px] focus:shadow-black data-[state=checked]:bg-black outline-none cursor-default"
				id="bounce"
				checked={bounce}
				onCheckedChange={e => setBounce(e)}
			>
				<Switch.Thumb className="block w-[21px] h-[21px] bg-white rounded-full shadow-[0_2px_2px] shadow-blackA4 transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]" />
			</Switch.Root>
		</Label>
		<input type="submit" value="Export"
			className="bg-[#444] text-white rounded-lg p-2"
			onClick={e => {
				e.preventDefault();
				submitHandler({ size, fps, speed, loop_gif: loop, bounce })
			}}
		/>
	</form>


}