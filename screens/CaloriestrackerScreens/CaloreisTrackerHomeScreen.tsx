import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";
import React, { useEffect, useState } from "react";
import { View } from "react-native";
import { CaloriesTrackerNeedScreen } from "./CaloriesTrackerNeedScreen";
import { CaloriesTrackerLogScreen } from "./CaloriesTrackerLogScreen";
import { getInfoEntries } from "../../database/InfosDB";
import { getLastWeight } from "../../database/WeightTrackerDB";

const Tab = createMaterialTopTabNavigator();

export const CaloriesTrackerHomeScreen: React.FC = () => {
  const [height, setHeight] = useState<number>(0);
  const [age, setAge] = useState<number>(0);
  const [isMale, setIsMale] = useState<boolean>(true);
  const [weight, setWeight] = useState<number>(0);
  const [bodyFatPercentage, setBodyFatPercentage] = useState<number>(0);

  useEffect(() => {
    getInfoEntries().then((entries) => {
      const dateOfBirth = entries.get(2)?.fieldValue;
      if (dateOfBirth) {
        setAge(new Date().getFullYear() - new Date(dateOfBirth).getFullYear());
      }
      //setHeight(Number(entries.get(3)?.fieldValue));
      //set sex
    });
    getLastWeight().then((weight) => {
      setWeight(weight.weight);
    });
    // getLastBF
    console.log(height);
  }, []);

  const initialParam = {
    height: height,
    age: age,
    isMale: isMale,
    weight: weight,
    bodyFatPercentage: bodyFatPercentage,
  };

  return (
    <View style={{ flex: 1 }}>
      <Tab.Navigator initialRouteName={"CaloriesTrackerHomeScreen"}>
        <Tab.Screen
          name="Need"
          component={CaloriesTrackerNeedScreen}
          initialParams={initialParam}
        />
        <Tab.Screen
          name="Log"
          component={CaloriesTrackerLogScreen}
          initialParams={initialParam}
        />
      </Tab.Navigator>
    </View>
  );
};
