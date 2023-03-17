import { createMaterialTopTabNavigator } from "@react-navigation/material-top-tabs";
import React, { useEffect, useState } from "react";
import { View } from "react-native";
import { CaloriesTrackerNeedScreen } from "./CaloriesTrackerNeedScreen";
import { CaloriesTrackerLogScreen } from "./CaloriesTrackerLogScreen";
import { getInfoEntries } from "../../database/InfosDB";
import { getLastWeight } from "../../database/WeightTrackerDB";
import { StackNavigationProp } from "@react-navigation/stack";
import { RouteProp } from "@react-navigation/native";

const Tab = createMaterialTopTabNavigator();

type ParamList = {
  Need: {
    height: number;
    age: number;
    isMale: boolean;
    weight: number;
    bodyFatPercentage: number;
  };
};

export type NavigationPropsCaloriesTrackerPages = {
  navigation: StackNavigationProp<ParamList, "Need">;
  route: RouteProp<ParamList, "Need">;
};

export const CaloriesTrackerHomeScreen: React.FC = () => {
  const [height, setHeight] = useState<number>(0);
  const [age, setAge] = useState<number>(0);
  const [isMale, setIsMale] = useState<boolean>(true);
  const [weight, setWeight] = useState<number>(0);
  const [bodyFatPercentage, setBodyFatPercentage] = useState<number>(0);

  useEffect(() => {
    getInfoEntries()
      .then((entries) => {
        const dateOfBirth = entries.get(2)?.fieldValue;
        if (dateOfBirth) {
          setAge(
            new Date().getFullYear() - new Date(dateOfBirth).getFullYear()
          );
        }
        //setHeight(Number(entries.get(3)?.fieldValue));
        //set sex
      })
      .catch(() => console.debug("error fetching info entries"));
    getLastWeight()
      .then((weight) => {
        setWeight(weight.weight);
      })
      .catch(() => console.debug("error fetching last weight"));
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
