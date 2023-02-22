import React from "react";
import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";

const NativeStackNavigator = createNativeStackNavigator();

export default function App() {
  return (
    <NavigationContainer>
      <NativeStackNavigator.Navigator initialRouteName="Home">
      </NativeStackNavigator.Navigator>
    </NavigationContainer>
  );
}
