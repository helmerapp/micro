const TimerDisplay = ({ seconds, isVisible }: { seconds: number, isVisible: boolean }) => {
    return (
        <div id="timer" className={isVisible ? "": "hidden"}>
            00:{seconds <= 9 ? '0' + seconds : seconds}
        </div>
    );
};

export default TimerDisplay;
