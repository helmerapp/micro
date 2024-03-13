import { useEffect, useRef } from "react"
import "./knobs.css"
import Knob from './Knob';
import { createKnobCSS } from './KnobHelper';

export default function KnobEl() {

	const rotateCssKnobRef = useRef(null);
	const knobInstance = useRef(null);

	useEffect(() => {
		console.log("creating knob")
		const inputEl = rotateCssKnobRef.current;
		const callback = (knob, indicator) => {
			// Handle knob and indicator rendering or any other operations
			console.log('Knob callback:', knob, indicator);
		};

		knobInstance.current = new Knob(inputEl, callback);

		createKnobCSS(rotateCssKnobRef.current, 'rotate-css-knob');

		return () => {
			// Clean up any event listeners or resources if needed
		};
	}, []);

	return <input
		ref={rotateCssKnobRef}
		id="rotate-css-knob"
		name="rotate-css-knob"
		type="range"
		value="50"
		min="0"
		max="100"
		data-angle-start="-200"
		data-angle-end="200"
		data-indicator-auto-rotate="true"
	/>
}