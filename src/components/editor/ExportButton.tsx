

export default function ExportButton({ clickHandler, exporting }: {
	clickHandler: () => void,
	exporting: boolean
}) {

	return <input type="submit" value={exporting ? "Exporting..." : "Export to Desktop"}
		className="bg-[#444] text-white rounded-lg p-2 cursor-pointer"
		onClick={e => {
			e.preventDefault();
			if (!exporting) clickHandler()
		}}
	/>

}