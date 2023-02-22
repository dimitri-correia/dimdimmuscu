import Icon from "@mdi/react";
import { mdiScale, mdiScaleBathroom } from "@mdi/js";

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
    <Icon path={mdiScaleBathroom} size={1} />
  ),
  new ScreenItem(
    "Calories Tracker",
    "CaloriesTracker",
    <Icon path={mdiScale} size={1} />
  ),
];
