import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import RecordingButton from "./RecordingButton";
import TimerDisplay from "./TimerDisplay";
import "./toolbar.css";
import useInterval from "../../utils/useInterval";

const RECORDING_LIMIT = 20;

const ToolbarReact = () => {
  const [minutes, setMinutes] = useState(0);
  const [seconds, setSeconds] = useState(RECORDING_LIMIT);
  const [time, setTime] = useState(
    `${minutes <= 9 ? "0" : ""}${minutes}:${seconds <= 9 ? "0" : ""}${seconds}`
  );
  const [isVisible, setIsVisible] = useState(false);
  const [timer, setTimer] = useState<any>(null);

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

  useEffect(() => {
    setTime(
      `${minutes <= 9 ? "0" : ""}${minutes}:${
        seconds <= 9 ? "0" : ""
      }${seconds}`
    );
  }, [minutes, seconds]);

  const updateSeconds = () => {
    if (seconds === 0) {
      stopTimer();
      setSeconds(RECORDING_LIMIT);
    } else {
      setSeconds(seconds - 1);
    }
  };

  const startTimer = () => {
    setIsVisible(true);
    const interval = useInterval(updateSeconds, 1000);
    console.log("interval", interval);
    setTimer(interval);
  };

  const stopTimer = () => {
    setIsVisible(false);
    clearInterval(timer);
  };

  const onStartRecording = async () => {
    try {
      startTimer();
      await invoke("start_capture");
    } catch (error) {
      console.error("Error starting recording:", error);
    }
  };

  const onStopRecording = async () => {
    try {
      await invoke("stop_capture", {});
      stopTimer();
      setSeconds(RECORDING_LIMIT);
      setTime(
        `${minutes <= 9 ? "0" : ""}${minutes}:${
          seconds <= 9 ? "0" : ""
        }${seconds}`
      );
    } catch (error) {
      console.error("Error stopping recording:", error);
    }
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
