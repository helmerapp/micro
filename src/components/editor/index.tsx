import { useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri'

export default function Editor() {

	useEffect(() => {
		const form = document.getElementById('form') as HTMLFormElement;
		const size = document.getElementById('size') as HTMLSelectElement;
		const fps = document.getElementById('fps') as HTMLInputElement;
		const speed = document.getElementById('speed') as HTMLInputElement;
		const loop = document.getElementById('loop') as HTMLInputElement;
		const bounce = document.getElementById('bounce') as HTMLInputElement;

		form.addEventListener("submit", (event) => {
			event.preventDefault();

			const options = {
				size: size.value,
				fps: fps.value,
				speed: Number.parseFloat(speed.value),
				loop_gif: loop.checked,
				bounce: bounce.checked
			}

			// send data to server
			invoke('export_handler', {
				options
			}).then(() => {
				console.log("export started")
			})
		});
	}, [])

	return (
		<main>
			{/* Instead of this Canvas, GIF preview will be here */}
			<canvas id="canvas" width="640" height="480"></canvas>

			<form id="form" className="panel">
				<div>
					<label htmlFor="size">Size</label>
					<select name="size" id="size">
						<option value="200">200</option>
						<option value="400">400</option>
						<option value="800">800</option>
						<option value="1000" selected>1000</option>
						<option value="2000">2000</option>
					</select>
				</div>
				<div>
					<label htmlFor="fps">Smoothness</label>
					<input id="fps" type="number" min="15" max="60" value="30" />
				</div>
				<div>
					<label htmlFor="speed">Speed</label>
					<input id="speed" type="range" min="0.5" max="2" value="1" step="0.1" />
				</div>
				<div>
					<label htmlFor="loop">Loop</label>
					<input id="loop" type="checkbox" />
				</div>
				<div>
					<label htmlFor="bounce">Bounce</label>
					<input id="bounce" type="checkbox" checked />
				</div>
				<input id="export" type="submit" value="Export" />
			</form>
		</main>
	);
}