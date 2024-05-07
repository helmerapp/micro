const DragHandle = () => {
  return (
    <div className="z-50 h-full cursor-grabbing text-white opacity-40 pl-1 pr-1 pb-1 pt-1" data-tauri-drag-region>
			<svg width="12" height="20" viewBox="0 0 12 20" fill="none" xmlns="http://www.w3.org/2000/svg"
				style={{pointerEvents: "none"}}>
				<g clip-path="url(#clip0_4_2)">
					<circle cx="2" cy="2" r="2" fill="white" fill-opacity="0.4" />
					<circle cx="2" cy="10" r="2" fill="white" fill-opacity="0.4" />
					<circle cx="2" cy="18" r="2" fill="white" fill-opacity="0.4" />
					<circle cx="10" cy="2" r="2" fill="white" fill-opacity="0.4" />
					<circle cx="10" cy="10" r="2" fill="white" fill-opacity="0.4" />
					<circle cx="10" cy="18" r="2" fill="white" fill-opacity="0.4" />
				</g>
				<defs>
					<clipPath id="clip0_4_2">
						<rect width="12" height="20" fill="white" />
					</clipPath>
				</defs>
			</svg>
		</div>
  )
}

export default DragHandle