import Spinner from "./Spinner";

export default function ExportButton({ clickHandler, exporting }: {
	clickHandler: () => void,
	exporting: boolean
}) {

	return <button className="w-[280px] outline-none cursor-pointer bg-[orange] border-none px-6 py-4 m-0 inline-block relative uppercase tracking-wider font-bold text-sm rounded-xl overflow-hidden before:absolute before:inset-0 before:bg-[rgba(255,255,255,0.2)] before:blur-md before:skew-x-12 before:-translate-x-full before:transition-transform before:duration-300 before:ease-in-out hover:before:translate-x-full mb-5"
		onClick={e => {
			e.preventDefault();
			if (!exporting) clickHandler()
		}}
		style={{
			cursor: exporting ? "wait" : "pointer"
		}}
	>
		{
			exporting
				? <Spinner size={20} />
				: <span className="text-black">Export</span>
		}
	</button>
}
