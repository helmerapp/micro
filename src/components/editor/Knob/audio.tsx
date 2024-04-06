// const [audioContext, setAudioContext] = useState<null | AudioContext>(null);

// 	const playClickSound = () => {
// 		if (!audioContext) return;

// 		const buffer = audioContext.createBuffer(1, audioContext.sampleRate * 0.02, audioContext.sampleRate);
// 		const data = buffer.getChannelData(0);

// 		for (let i = 0; i < buffer.length; i++) {
// 			data[i] = Math.random() * 2 - 1; // Generate random noise
// 		}

// 		const source = audioContext.createBufferSource();
// 		source.buffer = buffer;

// 		const filter = audioContext.createBiquadFilter();
// 		filter.type = 'bandpass';
// 		filter.frequency.value = 3000;

// 		const gainNode = audioContext.createGain();
// 		gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);

// 		source.connect(filter);
// 		filter.connect(gainNode);
// 		gainNode.connect(audioContext.destination);

// 		source.start();
// 		gainNode.gain.exponentialRampToValueAtTime(0.00001, audioContext.currentTime + 0.02);
// 		source.stop(audioContext.currentTime + 0.02);
// 	};

// 	useEffect(() => {
// 		// @ts-expect-error: webkitAudioContext is not in the lib but needed for iOS / Safari
// 		const context = new (window.AudioContext || window.webkitAudioContext)();
// 		setAudioContext(context);
// 	}, []);

// if (!audioContext) return;

// audioContext