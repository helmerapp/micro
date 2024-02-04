import { invoke } from '@tauri-apps/api/tauri'
import Preview from "./Preview";
import Controls from "./Controls";

export default function Editor() {

	const exportHandler = (options: {
		size: number,
		fps: number,
		speed: number,
		loop_gif: boolean,
		bounce: boolean
	}) => {
		invoke('export_handler', {
			options
		}).then(() => {
			console.log("export started")
		})
	}

	return (
		<main className="w-full h-full flex flex-col bg-[#222] p-8 items-center">
			<Preview />
			<Controls submitHandler={exportHandler} />
		</main>
	);
}