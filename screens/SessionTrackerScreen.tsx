import React, { useEffect, useState } from "react";
import { Button, Text, TextInput, View } from "react-native";
import {
  addCardio,
  CardioEntry,
  refreshCardioEntries,
} from "../logic/CardioLogic";
import CommonStyles from "../styles/CommonStyles";
import { addWeightStyles, pageStyles } from "../styles/WeightTrackerStyles";
import * as TextWT from "../assets/texts/WeightTracker";

interface Set {
  setNumber: number;
  repDone: number;
  weightLifted: number;
}

export const SessionTrackerScreen: React.FC = () => {
  const [cardioEntries, setCardioEntries] = useState<CardioEntry[]>([]);

  const [setList, setSetList] = useState<Set[]>([]);

  // table with id, ex, date
  // table with id, idExDate, set, rep, weight

  useEffect(() => {
    refreshCardioEntries(setCardioEntries);
  }, []);

  return (
    <View style={CommonStyles.container}>
      <View style={addWeightStyles.addWeightContainer}>
        <Text style={addWeightStyles.addWeightLabel}>{TextWT.enterWeight}</Text>
        <TextInput
          style={addWeightStyles.addWeightInput}
          onChangeText={(text) => setNewName(text)}
        />
        <TextInput
          style={addWeightStyles.addWeightInput}
          keyboardType="numeric"
          onChangeText={(text) => setNewTime(text)}
        />
        <TextInput
          style={addWeightStyles.addWeightInput}
          keyboardType="numeric"
          onChangeText={(text) => setNewCalo(text)}
        />
        <Button
          title={TextWT.add}
          onPress={() => {
            addCardio(newName, newTime, newCalo);
            refreshCardioEntries(setCardioEntries);
          }}
        />
      </View>
      <Row
        date={TextWT.date}
        name={TextWT.weightInKg}
        time={TextWT.average7}
        calo={TextWT.average31}
      />
      {cardioEntries.map((entry) => (
        <Row
          date={entry.date.toDateString()}
          name={entry.name}
          time={entry.time.toFixed(0)}
          calo={entry.calo.toFixed(0)}
        />
      ))}
    </View>
  );
};

interface RowItem {
  date: string;
  name: string;
  time: string;
  calo: string;
}

const Row = ({ date, name, time, calo }: RowItem) => {
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{date}</Text>
      <Text style={pageStyles.column}>{name}</Text>
      <Text style={pageStyles.column}>{time}</Text>
      <Text style={pageStyles.column}>{calo}</Text>
    </View>
  );
};
