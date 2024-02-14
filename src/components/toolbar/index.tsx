import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import RecordingButton from "./RecordingButton";
import TimerDisplay from "./TimerDisplay";
import "./toolbar.css";
import useInterval from "../../utils/useInterval";

const RECORDING_LIMIT = 20;

const ToolbarReact = () => {
  const [seconds, setSeconds] = useState(RECORDING_LIMIT);
  const [isVisible, setIsVisible] = useState(false);

  useInterval(() => {
    if (isVisible && seconds > 0) {
      setSeconds((prevSeconds) => prevSeconds - 1);
    }
  }, 1000);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape" || event.key === "Esc") {
        // TODO implement hide cropper window!
        invoke("hide_toolbar");
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  const startTimer = () => {
    setIsVisible(true);
  };

  const stopTimer = () => {
    setIsVisible(false);
    setSeconds(RECORDING_LIMIT);
  };

  const onStartRecording = () => {
    invoke("start_capture");
    startTimer();
  };

  const onStopRecording = () => {
    invoke("stop_capture", {});
    stopTimer();
  };

  return (
    <main>
      <RecordingButton
        onStartRecording={onStartRecording}
        onStopRecording={onStopRecording}
      />
      <TimerDisplay seconds={seconds} isVisible={isVisible} />
    </main>
  );
};

export default ToolbarReact;
