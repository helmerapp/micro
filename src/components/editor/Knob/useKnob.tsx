import { useState, useEffect, useRef } from 'react';

function setupKnob(knob, container) {

	// var rect = container.getBoundingClientRect();
	// knob.setPosition(rect.left + container.clientLeft, rect.top + container.clientTop);
	knob.setPosition(container.offsetLeft, container.offsetTop);
	knob.setDimensions(container.clientWidth, container.clientHeight);

	// Detect touch capable client
	if ('ontouchstart' in window) {

		container.addEventListener('touchstart', function (e) {
			// reset the position in case knob moved
			knob.setPosition(container.offsetLeft, container.offsetTop);

			const timeStamp = e.timeStamp.getTime ? e.timeStamp.getTime() : e.timeStamp;
			// Keep track of the knobs currently being touched to support multitouch.
			activeKnobs[e.targetTouches[0].identifier] = knob;
			knob.doTouchStart(e.targetTouches, timeStamp);
			e.preventDefault();
		}, false);

		document.addEventListener('touchmove', function (e) {
			const timeStamp = e.timeStamp.getTime ? e.timeStamp.getTime() : e.timeStamp;
			// Support multi-touch knobs by only passing the appropriate touch events.
			for (let i = 0, l = e.changedTouches.length; i < l; i++) {
				const k = activeKnobs[e.changedTouches[i].identifier];
				if (typeof k !== "undefined") {
					k.doTouchMove([e.changedTouches[i]], timeStamp, e.scale);
				}
			}

		}, false);

		document.addEventListener('touchend', function (e) {
			const timeStamp = e.timeStamp.getTime ? e.timeStamp.getTime() : e.timeStamp;
			knob.doTouchEnd(timeStamp);
		}, false);

		document.addEventListener('touchcancel', function (e) {
			const timeStamp = e.timeStamp.getTime ? e.timeStamp.getTime() : e.timeStamp;
			knob.doTouchEnd(timeStamp);
		}, false);

	} else {
		// No touch capable client detected, use mouse interactions
		let mousedown = false;
		const naturalScrolling = !/win|linux/.test(getPlatform().toLowerCase())

		container.addEventListener('mousedown', function (e) {
			// reset the position in case knob moved
			knob.setPosition(container.offsetLeft, container.offsetTop);

			knob.doTouchStart([{
				pageX: e.pageX,
				pageY: e.pageY
			}], e.timeStamp);

			mousedown = true;
		}, false);

		document.addEventListener('mousemove', function (e) {
			if (!mousedown) { return; }

			knob.doTouchMove([{
				pageX: e.pageX,
				pageY: e.pageY
			}], e.timeStamp);

			mousedown = true;

			// Prevent selection on drag
			if (e.preventDefault)
				e.preventDefault();
			e.returnValue = false;
		}, false);

		document.addEventListener('mouseup', function (e) {
			if (!mousedown) { return; }

			knob.doTouchEnd(e.timeStamp);

			mousedown = false;
		}, false);

		// Handle scroll for webkit
		container.addEventListener('mousewheel', function (e) {
			// reset the position in case knob moved
			knob.setPosition(container.offsetLeft, container.offsetTop);

			const delta = naturalScrolling ? e.wheelDelta : -e.wheelDelta
			knob.doMouseScroll(delta, e.timeStamp, e.pageX, e.pageY);
			// Prevent page scroll
			if (e.preventDefault)
				e.preventDefault();
			e.returnValue = false;
		}, false);

		// Handle scroll for gecko
		// container.addEventListener('MozMousePixelScroll', function(e) {
		container.addEventListener('DOMMouseScroll', function (e) {
			// reset the position in case knob moved
			knob.setPosition(container.offsetLeft, container.offsetTop);
			let delta = -4 * e.detail
			delta = naturalScrolling ? -delta : -delta
			knob.doMouseScroll(delta, e.timeStamp, e.pageX, e.pageY);
			// Prevent page scroll
			if (e.preventDefault)
				e.preventDefault();
			e.returnValue = false;
		}, false);
	}
}

function useKnob(inputEl, containerClass, callback) {
	const [knob, setKnob] = useState(null);
	const containerRef = useRef(null);

	useEffect(() => {
		if (containerRef.current) {
			const knobInstance = setupKnob(inputEl, containerRef.current);
			setKnob(knobInstance);
			callback(knobInstance.__value);
		}
	}, [containerRef, inputEl, callback]);

	return { knob, containerRef };
}