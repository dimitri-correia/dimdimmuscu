import React from "react";
import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import { HomeScreen } from "./screens/HomeScreen";

const NativeStackNavigator = createNativeStackNavigator();

export default function App() {
  return (
    <NavigationContainer>
      <NativeStackNavigator.Navigator initialRouteName="Home">
        <NativeStackNavigator.Screen
          name={"Home"}
          component={HomeScreen}
          // options={{ headerShown: false }}
        />
      </NativeStackNavigator.Navigator>
    </NavigationContainer>
  );
}
