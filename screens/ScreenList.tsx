import React from "react";
import { Image } from "react-native";
import HomeScreenStyles from "../styles/HomeScreenStyles";

export class ScreenItem {
  name: string;
  screenName: string;
  icon: JSX.Element;

  constructor(name: string, screenName: string, icon: JSX.Element) {
    this.name = name;
    this.screenName = screenName;
    this.icon = icon;
  }
}

export default [
  new ScreenItem(
    "Weight Tracker",
    "WeightTracker",
    (
      <Image
        source={require("../assets/ScreenIcons/WeightTracker.png")}
        style={HomeScreenStyles.itemImage}
      />
    )
  ),
  new ScreenItem(
    "Calories Tracker",
    "CaloriesTracker",
    (
      <Image
        source={require("../assets/ScreenIcons/CaloriesTracker.png")}
        style={HomeScreenStyles.itemImage}
      />
    )
  ),
  new ScreenItem(
    "Exercises List",
    "ExercisesList",
    (
      <Image
        source={require("../assets/ScreenIcons/ExercisesList.png")}
        style={HomeScreenStyles.itemImage}
      />
    )
  ),
  new ScreenItem(
    "Personal Record",
    "PersonalRecord",
    (
      <Image
        source={require("../assets/ScreenIcons/PersonalRecord.png")}
        style={HomeScreenStyles.itemImage}
      />
    )
  ),
];
