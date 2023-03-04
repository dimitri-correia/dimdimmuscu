import React from "react";
import { View } from "react-native";
import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";

const Tab = createMaterialTopTabNavigator();

export const CalculatorsScreen: React.FC = () => {
  return (
    <Tab.Navigator initialRouteName={"CalculatorsScreen"}>
      <Tab.Screen name="1RMCalculator" component={OneRMCalculator} />
      <Tab.Screen name="WarmUpMaxAttempt" component={WarmUpMaxAttempt} />
    </Tab.Navigator>
  );
};

const OneRMCalculator: React.FC = () => {
  return <View />;
};
const WarmUpMaxAttempt: React.FC = () => {
  return <View />;
};
