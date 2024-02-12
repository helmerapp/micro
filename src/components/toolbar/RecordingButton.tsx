import { useState } from 'react';

const RecordingButton = ({ onStartRecording, onStopRecording }: { onStartRecording: () => void, onStopRecording: () => void }) => {
    const [isRecording, setIsRecording] = useState(false);

    const handleClick = () => {
        console.log({isRecording});
        if (isRecording) {
            onStopRecording();
        } else {
            onStartRecording();
        }
        setIsRecording(!isRecording);
    };

    return (
        <button className="record" onClick={handleClick}>
            {isRecording ? (
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round">
                    <rect x="2" y="2" width="20" height="20"></rect>
                </svg>
            ) : (
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round">
                    <circle cx="12" cy="12" r="10"></circle>
                </svg>
            )}
        </button>
    );
};

export default RecordingButton;
