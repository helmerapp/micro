import { useEffect, useRef } from 'react';

const useInterval = (callback: () => void, delay: number) => {
    const savedCallback = useRef<() => void>();
    const timer = useRef<number | null>(null);

    useEffect(() => {
        savedCallback.current = callback;
    }, [callback]);

    useEffect(() => {
        function tick() {
            savedCallback.current!();
        }
        timer.current = setInterval(tick, delay);
        return () => {
            if (timer.current !== null) {
                clearInterval(timer.current);
                timer.current = null;
            }
        };
    }, [delay]);

    return timer.current;
};

export default useInterval;
