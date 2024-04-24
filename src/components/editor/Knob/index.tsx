import { useEffect, useRef, useState } from "react"
import "./knobs.css"
import { KnobHelper } from "./KnobHelper";

export default function KnobEl() {

	const knobRef = useRef(null);
	const [prevValue, setPrevValue] = useState(0);
	const [knobValue, setKnobValue] = useState(0);
	const [audioContext, setAudioContext] = useState<null | AudioContext>(null);

	const playClickSound = () => {
		if (!audioContext) return;

		const buffer = audioContext.createBuffer(1, audioContext.sampleRate * 0.02, audioContext.sampleRate);
		const data = buffer.getChannelData(0);

		for (let i = 0; i < buffer.length; i++) {
			data[i] = Math.random() * 2 - 1; // Generate random noise
		}

		const source = audioContext.createBufferSource();
		source.buffer = buffer;

		const filter = audioContext.createBiquadFilter();
		filter.type = 'bandpass';
		filter.frequency.value = 3000;

		const gainNode = audioContext.createGain();
		gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);

		source.connect(filter);
		filter.connect(gainNode);
		gainNode.connect(audioContext.destination);

		source.start();
		gainNode.gain.exponentialRampToValueAtTime(0.00001, audioContext.currentTime + 0.02);
		source.stop(audioContext.currentTime + 0.02);
	};

	useEffect(() => {
		// @ts-expect-error: webkitAudioContext is not in the lib but needed for iOS / Safari
		const context = new (window.AudioContext || window.webkitAudioContext)();
		setAudioContext(context);
	}, []);

	useEffect(() => {
		if (!knobRef.current) return;
		if (!audioContext) return;

		KnobHelper.createKnobCSS(knobRef.current, "rotate-css-knob", (value) => {
			const currentValue = parseInt(value, 10);
			setKnobValue(currentValue);

			// console.log("currentValue:", currentValue);
			// console.log("prevValue:", prevValue);

			if (currentValue !== prevValue) {
				setPrevValue(currentValue);
			}
		})

	}, [audioContext]);

	useEffect(() => {
		playClickSound();
		console.log("prevValue:", prevValue);
	}, [prevValue]);

	return <input
		ref={knobRef}
		id="rotate-css-knob"
		name="rotate-css-knob"
		type="range"
		defaultValue={0}
		// value={knobValue}
		// value="50"
		min="0"
		max="20"
		step="1"
		data-angle-start="0"
		data-angle-end="270"
		data-indicator-auto-rotate="true"
	/>
}