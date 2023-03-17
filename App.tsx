import React from "react";
import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";

import { HomeScreen } from "./screens/HomeScreen";
import { WeightTracker } from "./screens/WeightTracker";
import { ExercisesList } from "./screens/ExercisesList";
import { PersonalRecordScreen } from "./screens/PersonalRecordScreen";
import { CalculatorsScreen } from "./screens/CalucatorsScreens/CalculatorsScreen";
import { InfosScreen } from "./screens/InfosScreen";
import { PictureEvolutionScreen } from "./screens/PictureEvolutionScreen";
import { CaloriesTrackerHomeScreen } from "./screens/CaloriestrackerScreens/CaloriesTrackerHomeScreen";

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
        <NativeStackNavigator.Screen
          name="WeightTracker"
          component={WeightTracker}
        />
        <NativeStackNavigator.Screen
          name={"CaloriesTracker"}
          component={CaloriesTrackerHomeScreen}
        />
        <NativeStackNavigator.Screen
          name="ExercisesList"
          component={ExercisesList}
        />
        <NativeStackNavigator.Screen
          name="PersonalRecord"
          component={PersonalRecordScreen}
        />
        <NativeStackNavigator.Screen
          name="Calculators"
          component={CalculatorsScreen}
        />
        <NativeStackNavigator.Screen name="Infos" component={InfosScreen} />
        <NativeStackNavigator.Screen
          name="PictureEvolution"
          component={PictureEvolutionScreen}
        />
      </NativeStackNavigator.Navigator>
    </NavigationContainer>
  );
}
