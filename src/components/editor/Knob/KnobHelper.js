const KnobHelper = {};
(function (undefined) {
	const members = {
		createKnobCSS: function (inputEl, containerClass) {
			const knob = new Knob(inputEl,
				function (knob, indicator) {
					KnobHelper.drawKnobCSS(knob, indicator);
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
		},

	} // end members

	for (const key in members) {
		KnobHelper[key] = members[key];
	}

})();

export const createKnobCSS = KnobHelper.createKnobCSS;