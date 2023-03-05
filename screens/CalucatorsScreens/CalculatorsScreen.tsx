import React from "react";
import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";
import { OneRMCalculator } from "./OneRMCalculator";
import { WarmUpMaxAttempt } from "./WarmUpMaxAttempt";

const Tab = createMaterialTopTabNavigator();

export const CalculatorsScreen: React.FC = () => {
  return (
    <Tab.Navigator initialRouteName={"CalculatorsScreen"}>
      <Tab.Screen name="1RMCalculator" component={OneRMCalculator} />
      <Tab.Screen name="WarmUpMaxAttempt" component={WarmUpMaxAttempt} />
    </Tab.Navigator>
  );
};
