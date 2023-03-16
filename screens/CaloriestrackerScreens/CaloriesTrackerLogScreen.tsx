import React, { useState } from "react";
import { Button, FlatList, Text, TextInput, View } from "react-native";
import * as TextWT from "../../assets/texts/WeightTracker";
import {
  calculatePercentage,
  CaloEntry,
} from "../../logic/CaloriesTrackerLogic";
import CaloStyles from "../../styles/CaloStyles";
import { addWeightStyles } from "../../styles/WeightTrackerStyles";

export const CaloriesTrackerLogScreen: React.FC = () => {
  const [foodName, setFoodName] = useState("");
  const [foodQuantity, setFoodQuantity] = useState("");
  const [foodCalories, setFoodCalories] = useState("");
  const [foodProteins, setFoodProteins] = useState("");

  const [entries, setEntries] = useState<CaloEntry[]>([]);

  return (
    <>
      {getView(setFoodName, setFoodQuantity, setFoodCalories, setFoodProteins)}
      <View>
        <View style={CaloStyles.header}>
          <Text style={CaloStyles.cell}>Name</Text>
          <Text style={CaloStyles.cell}>Calories</Text>
          <Text style={CaloStyles.cell}>% Cal</Text>
          <Text style={CaloStyles.cell}>Protein</Text>
          <Text style={CaloStyles.cell}>% Prot</Text>
        </View>
        <FlatList
          data={entries}
          renderItem={renderItem}
          keyExtractor={(_, index) => index.toString()}
        />
      </View>
    </>
  );
};

function getView(
  setFoodName: (value: ((prevState: string) => string) | string) => void,
  setFoodQuantity: (value: ((prevState: string) => string) | string) => void,
  setFoodCalories: (value: ((prevState: string) => string) | string) => void,
  setFoodProteins: (value: ((prevState: string) => string) | string) => void
) {
  return (
    <>
      <View style={addWeightStyles.addWeightContainer}>
        <Text>Name</Text>
        <Text>Qtt (g)</Text>
        <Text>Cal (100g)</Text>
        <Text>Prot (100g)</Text>
      </View>
      <View style={addWeightStyles.addWeightContainer}>
        <TextInput
          style={addWeightStyles.addWeightInput}
          onChangeText={setFoodName}
        />
        <TextInput
          style={addWeightStyles.addWeightInput}
          keyboardType="numeric"
          onChangeText={setFoodQuantity}
        />
        <TextInput
          style={addWeightStyles.addWeightInput}
          keyboardType="numeric"
          onChangeText={setFoodCalories}
        />
        <TextInput
          style={addWeightStyles.addWeightInput}
          keyboardType="numeric"
          onChangeText={setFoodProteins}
        />
        <Button
          title={TextWT.add}
          onPress={() => {
            console.log("cc");
            //addWeight(weightEntries, newWeight);
            //refreshWeightEntries(setWeightEntries);
          }}
        />
      </View>
    </>
  );
}

const renderItem = ({ item }: { item: CaloEntry }) => {
  const caloriesPercentage = calculatePercentage(item.calo, 3300);
  const proteinPercentage = calculatePercentage(item.prot, 550);
  return (
    <View style={CaloStyles.row}>
      <Text style={CaloStyles.cell}>{item.name}</Text>
      <Text style={CaloStyles.cell}>{item.calo}</Text>
      <Text style={CaloStyles.cell}>{caloriesPercentage}%</Text>
      <Text style={CaloStyles.cell}>{item.prot}</Text>
      <Text style={CaloStyles.cell}>{proteinPercentage}%</Text>
    </View>
  );
};
