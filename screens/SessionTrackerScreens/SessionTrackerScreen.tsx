import React from "react";
import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";
import { SessionTrackerScreenLog } from "./SessionTrackerScreenLog";

const Tab = createMaterialTopTabNavigator();
export const SessionTrackerScreen: React.FC = () => {
  return (
    <Tab.Navigator initialRouteName={"Log"}>
      <Tab.Screen name="Log" component={SessionTrackerScreenLog} />
    </Tab.Navigator>
  );
};
