import React, { useState } from "react";
import { StyleSheet, Text, View } from "react-native";
import {
  ActivityLevels,
  calculateBMR,
  calculateMuscularPotential,
  calculateWaterIntake,
  calculateWeightAndBMIStats,
} from "../../logic/CaloriesTrackerLogic";
import { Picker } from "@react-native-picker/picker";
import { NavigationPropsCaloriesTrackerPages } from "./CaloriesTrackerHomeScreen";

export const CaloriesTrackerNeedScreen: React.FC<
  NavigationPropsCaloriesTrackerPages
> = ({ navigation, route }) => {
  const {
    height: height,
    age: age,
    isMale: isMale,
    weight: weight,
    bodyFatPercentage: bodyFatPercentage,
  } = route.params;

  const [activityLevel, setActivityLevel] = useState<ActivityLevels>(
    ActivityLevels.SEDENTARY
  );

  const { bmi, maxHealthyWeight, minHealthyWeight } =
    calculateWeightAndBMIStats(age, weight, height);
  const bmr: number = calculateBMR(weight, height, age, isMale).mifflinStJeor;
  const calorieIntake: number = bmr * activityLevel;
  console.log(activityLevel);
  const waterIntake = calculateWaterIntake(weight);
  const muscularPotential: Record<number, number> = calculateMuscularPotential(
    weight,
    bodyFatPercentage
  );

  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.label}>Infos:</Text>
        <Text style={styles.value}>
          {`${height} cm, ${age} yo, ${
            isMale ? "Male" : "Female"
          }, ${weight} kg, ${bodyFatPercentage}% bfp`}
        </Text>
      </View>
      <View style={styles.header}>
        <Text style={styles.label}>Maintenance Calories:</Text>
        <Text style={styles.value}>{`${Math.round(
          calorieIntake
        )} daily => ${Math.round(calorieIntake * 7)} weekly`}</Text>
      </View>
      <View style={styles.header}>
        <Text style={styles.label}>Activity Level:</Text>
        <Picker
          selectedValue={activityLevel}
          onValueChange={(value: keyof typeof ActivityLevels) =>
            setActivityLevel(value)
          }
        >
          {Object.keys(ActivityLevels).map((key) => (
            <Picker.Item key={key} label={key} value={key} />
          ))}
        </Picker>

        <Text style={styles.value}>Activity Level: {activityLevel}</Text>
        <ActivityLevelTable bmr={bmr} />
      </View>
      <View style={styles.header}>
        <Text style={styles.label}>Muscular Potential:</Text>
        <Text style={styles.value}>
          At 5% body fat: {muscularPotential[5]} kg
        </Text>
        <Text style={styles.value}>
          At 10% body fat: {muscularPotential[10]} kg
        </Text>
        <Text style={styles.value}>
          At 15% body fat: {muscularPotential[15]} kg
        </Text>
      </View>
    </View>
  );
};

interface ActivityLevelTableProps {
  bmr: number;
}

const ActivityLevelTable = ({ bmr }: ActivityLevelTableProps) => {
  return (
    <View style={styles.table}>
      <View style={styles.tableRow}>
        <Text style={styles.tableHeaderCell}>Activity Level</Text>
        <Text style={styles.tableHeaderCell}>Multiplier</Text>
        <Text style={styles.tableHeaderCell}>Calories</Text>
      </View>
      {Object.entries(ActivityLevels).map(([key]) => (
        <View key={key} style={styles.tableRow}>
          <Text style={styles.tableCell}>{key}</Text>
          <Text style={styles.tableCell}>{Math.round(key * bmr)}</Text>
        </View>
      ))}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
  },
  row: {
    flexDirection: "row",
    justifyContent: "space-between",
    marginBottom: 20,
  },
  label: {
    fontSize: 20,
    fontWeight: "bold",
    marginBottom: 10,
  },
  value: {
    fontSize: 16,
  },
  table: {
    borderWidth: 1,
    borderColor: "#ddd",
    borderRadius: 4,
    marginTop: 10,
  },
  tableRow: {
    flexDirection: "row",
    borderBottomWidth: 1,
    borderColor: "#ddd",
  },
  tableHeaderCell: {
    padding: 10,
    fontWeight: "bold",
  },
  tableCell: {
    flex: 1,
    padding: 10,
    textAlign: "center",
  },
  header: {
    paddingVertical: 10,
    paddingHorizontal: 20,
  },
  headerText: {
    fontSize: 18,
  },
});
