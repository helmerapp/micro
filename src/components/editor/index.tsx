
import Preview from "./Preview";
import Controls from "./Controls";

export default function Editor() {
	return (
		<main className="w-full h-full flex flex-col bg-[#222] p-8">
			<Preview />
			<Controls />
		</main>
	);
}