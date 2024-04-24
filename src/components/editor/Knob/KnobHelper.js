import Knob from "./Knob";

const activeKnobs = {};

function getPlatform() {
	// 2022 way of detecting. Note : this userAgentData feature is available only in secure contexts (HTTPS)
	if (typeof navigator.userAgentData !== 'undefined' && navigator.userAgentData != null) {
		return navigator.userAgentData.platform;
	}
	// Deprecated but still works for most of the browser
	if (typeof navigator.platform !== 'undefined') {
		if (typeof navigator.userAgent !== 'undefined' && /android/.test(navigator.userAgent.toLowerCase())) {
			// android device’s navigator.platform is often set as 'linux', so let’s use userAgent for them
			return 'android';
		}
		return navigator.platform;
	}
	return 'unknown';
}

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

export const KnobHelper = {
	createKnobCSS: function (inputEl, containerClass, callback) {
		const knob = new Knob(inputEl,
			function (knob, indicator) {
				KnobHelper.drawKnobCSS(knob, indicator);
				callback(knob.__value);
			}),

			container = document.createElement('div'),
			body = document.createElement('div'),
			indicator = document.createElement('div');

		container.classList.add('ui-knob-container', containerClass);
		body.classList.add('ui-knob', 'ui-knob-shadow');
		indicator.classList.add('ui-knob-indicator');

		container.appendChild(body);
		container.appendChild(indicator);

		inputEl.style.display = 'none';
		inputEl.parentNode.insertBefore(container, inputEl);
		container.appendChild(inputEl);

		// center knob in container
		body.style.marginTop = -body.offsetHeight / 2 + 'px';
		body.style.marginLeft = -body.offsetWidth / 2 + 'px';

		setupKnob(knob, container);

		return knob;

	},

	drawKnobCSS: function (knob, indicator) {
		const container = knob.element.closest('.ui-knob-container');
		if (container) {
			const indicatorEl = container.querySelector('.ui-knob-indicator');
			if (indicatorEl) {
				indicatorEl.style.left = indicator.x - indicatorEl.offsetWidth / 2 + 'px';
				indicatorEl.style.top = indicator.y - indicatorEl.offsetHeight / 2 + 'px';

				const rotateText = `rotate(${(-indicator.angle)}deg)`;
				indicatorEl.style.transform = rotateText;
				indicatorEl.style.webkitTransform = rotateText;
				indicatorEl.style.mozTransform = rotateText;
				indicatorEl.style.oTransform = rotateText;
			}
		}
	}
};