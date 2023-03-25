import React, { useEffect, useState } from "react";
import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";
import { SessionTrackerScreenLog } from "./SessionTrackerScreenLog";
import { ExercisesEntry } from "../../logic/ExercisesListLogic";
import { getExerciseEntries } from "../../database/ExercisesListDB";
import { StackNavigationProp } from "@react-navigation/stack";
import { RouteProp } from "@react-navigation/native";

const Tab = createMaterialTopTabNavigator();

type SessionTrackerParams = {
  Log: {
    ex: ExercisesEntry[];
    id: number;
  };
};

export type NavigationPropsSessionTrackerPages = {
  navigation: StackNavigationProp<SessionTrackerParams, "Log">;
  route: RouteProp<SessionTrackerParams, "Log">;
};
export const SessionTrackerScreen: React.FC = () => {
  const [exerciseEntries, setExerciseEntries] = useState<ExercisesEntry[]>([]);

  useEffect(() => {
    getExerciseEntries()
      .then((ex: ExercisesEntry[]) => {
        setExerciseEntries(ex);
      })
      .catch(() => {
        console.debug("error fetching ex entries");
      });
  }, []);

  const initialParams = { ex: exerciseEntries, id: 2 };

  return (
    <Tab.Navigator initialRouteName={"Log"}>
      <Tab.Screen
        name="Log"
        component={SessionTrackerScreenLog}
        initialParams={initialParams}
      />
    </Tab.Navigator>
  );
};
