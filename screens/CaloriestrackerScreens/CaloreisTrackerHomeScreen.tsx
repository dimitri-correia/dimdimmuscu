import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";
import React from "react";
import { CaloriesTrackerNeedScreen } from "./CaloriesTrackerNeedScreen";
import { CaloriesTrackerLogScreen } from "./CaloriesTrackerLogScreen";

const Tab = createMaterialTopTabNavigator();

export const CaloriesTrackerHomeScreen: React.FC = () => {
  return (
    <Tab.Navigator initialRouteName={"CaloriesTrackerHomeScreen"}>
      <Tab.Screen name="Need" component={CaloriesTrackerNeedScreen} />
      <Tab.Screen name="Log" component={CaloriesTrackerLogScreen} />
    </Tab.Navigator>
  );
};
