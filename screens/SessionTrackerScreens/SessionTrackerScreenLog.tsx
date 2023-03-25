import React, { useRef, useState } from "react";
import { Text, TouchableOpacity, View } from "react-native";
import CommonStyles from "../../styles/CommonStyles";
import { pageStyles } from "../../styles/WeightTrackerStyles";
import * as TextWT from "../../assets/texts/WeightTracker";
import { Audio } from "expo-av";
import { useKeepAwake } from "expo-keep-awake";

interface Set {
  setNumber: number;
  repDone: number;
  weightLifted: number;
}

export const SessionTrackerScreenLog: React.FC = () => {
  useKeepAwake(); // prevent from sleeping
  // table with id, ex, date
  // table with id, idExDate, set, rep, weight
  return (
    <View style={CommonStyles.container}>
      <Chronometer />
      <Row
        date={TextWT.date}
        name={TextWT.weightInKg}
        time={TextWT.average7}
        calo={TextWT.average31}
      />
    </View>
  );
};

interface RowItem {
  date: string;
  name: string;
  time: string;
  calo: string;
}

const Row = ({ date, name, time, calo }: RowItem) => {
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{date}</Text>
      <Text style={pageStyles.column}>{name}</Text>
      <Text style={pageStyles.column}>{time}</Text>
      <Text style={pageStyles.column}>{calo}</Text>
    </View>
  );
};

const Chronometer = () => {
  const [time, setTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);
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
        if ((prevTime + 1) % 30 === 0) {
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
    <View style={{ flexDirection: "row", justifyContent: "space-around" }}>
      <Text>{formatTime(time)}</Text>
      <TouchableOpacity onPress={isRunning ? stopTimer : startTimer}>
        <Text>{isRunning ? "Stop" : "Start"}</Text>
      </TouchableOpacity>
      <TouchableOpacity onPress={resetTimer}>
        <Text>Reset</Text>
      </TouchableOpacity>
    </View>
  );
};
