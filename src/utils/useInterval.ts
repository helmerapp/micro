import { useEffect, useRef } from 'react';

const useInterval = (callback: () => void, delay: number) => {
    if (delay === 0) {
        return;
    }
    const savedCallback = useRef<() => void>();

    useEffect(() => {
        savedCallback.current = callback;
    }, [callback]);

    useEffect(() => {
        function tick() {
            savedCallback.current!();
        }
        const interval = setInterval(tick, delay);
        return () => {clearInterval(interval);};
    }, [delay]);
};

export default useInterval;
