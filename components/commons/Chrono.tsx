import React, { useRef, useState } from "react";
import { Audio } from "expo-av";
import { Text, TouchableOpacity, View } from "react-native";

export default () => {
  const [time, setTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [song, setSong] = useState<boolean>(true);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const formatTime = (time: number) => {
    const minutes = Math.floor(time / 60);
    const seconds = time % 60;
    return `${minutes}:${seconds < 10 ? `0${seconds}` : seconds}`;
  };

  const playSound = (): void => {
    const soundObject = new Audio.Sound();
    try {
      soundObject
        .loadAsync(require("../../assets/Songs/30sNotif.mp3"))
        .then(() => {
          soundObject.playAsync().catch(() => {
            console.log("Error playing 30s notif");
          });
        });
    } catch (error) {
      console.warn("Error playing 30s notif", error);
    }
  };

  const startTimer = () => {
    setIsRunning(true);
    intervalRef.current = setInterval(() => {
      setTime((prevTime) => {
        if (song && (prevTime + 1) % 30 === 0) {
          playSound();
        }
        return prevTime + 1;
      });
    }, 1000);
  };

  const stopTimer = () => {
    setIsRunning(false);
    clearInterval(intervalRef.current as NodeJS.Timeout);
  };

  const resetTimer = () => {
    stopTimer();
    setTime(0);
  };

  return (
    <View
      style={{
        flexDirection: "row",
        justifyContent: "space-around",
        padding: 10,
        backgroundColor: "grey",
      }}
    >
      <Text>{formatTime(time)}</Text>
      <TouchableOpacity onPress={isRunning ? stopTimer : startTimer}>
        <Text>{isRunning ? "Stop" : "Start"}</Text>
      </TouchableOpacity>
      <TouchableOpacity onPress={resetTimer}>
        <Text>Reset</Text>
      </TouchableOpacity>
      <TouchableOpacity onPress={() => setSong(!song)}>
        <Text>{song ? "Mute" : "Ring"}</Text>
      </TouchableOpacity>
    </View>
  );
};
