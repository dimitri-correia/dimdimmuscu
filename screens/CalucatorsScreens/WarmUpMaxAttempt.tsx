import React, { useState } from "react";
import { Text, TextInput, View } from "react-native";
import { maxAttemptStyles } from "../../styles/CalculatorsStyles";
import * as CalcuText from "../../assets/texts/CalculatorTexts";

interface WarmupSet {
  setNumber: number;
  percentage: number;
  weight: number;
  reps: number;
  restTime: number;
}

export const WarmUpMaxAttempt = () => {
  const [goal, setGoal] = useState<string>("");
  const [warmupSets, setWarmupSets] = useState<WarmupSet[]>([]);

  const computeWarmupSets = () => {
    const weights = [40, 50, 60, 70, 80, 90, 95];
    const reps = [8, 5, 4, 3, 2, 1, 1];
    const restTimes = [1, 2, 2, 2, 3, 3, 5];
    const maxWeight = parseInt(goal);

    const computedSets: WarmupSet[] = weights.map((weight, index) => {
      return {
        setNumber: index + 1,
        percentage: weight,
        weight: Math.round((maxWeight * weight) / 10) / 10,
        reps: reps[index],
        restTime: restTimes[index],
      };
    });

    setWarmupSets(computedSets);
  };

  return (
    <View style={maxAttemptStyles.container}>
      <Text style={maxAttemptStyles.title}>{CalcuText.enterGoal}:</Text>
      <TextInput
        style={maxAttemptStyles.input}
        keyboardType="numeric"
        value={goal}
        onChangeText={(text) => setGoal(text)}
        onSubmitEditing={computeWarmupSets}
      />
      {warmupSets.length > 0 ? (
        <View style={maxAttemptStyles.table}>
          <View style={maxAttemptStyles.row}>
            <Text style={maxAttemptStyles.heading}>Set #</Text>
            <Text style={maxAttemptStyles.heading}>%</Text>
            <Text style={maxAttemptStyles.heading}>{CalcuText.weight}</Text>
            <Text style={maxAttemptStyles.heading}>{CalcuText.reps}</Text>
            <Text style={maxAttemptStyles.heading}>{CalcuText.restTime}</Text>
          </View>
          {warmupSets.map(
            (set: {
              setNumber: number;
              percentage: number;
              weight: number;
              reps: number;
              restTime: number;
            }) => (
              <View key={set.setNumber} style={maxAttemptStyles.row}>
                <Text style={maxAttemptStyles.cell}>{set.setNumber}</Text>
                <Text style={maxAttemptStyles.cell}>{set.percentage}%</Text>
                <Text style={maxAttemptStyles.cell}>{set.weight}</Text>
                <Text style={maxAttemptStyles.cell}>{set.reps}</Text>
                <Text style={maxAttemptStyles.cell}>{set.restTime} min</Text>
              </View>
            )
          )}

          <View style={maxAttemptStyles.row}>
            <Text style={maxAttemptStyles.cell}>
              {CalcuText.maxAttempt} (100%)
            </Text>
            <Text style={maxAttemptStyles.cell}>{goal}x 1 rep</Text>
          </View>
        </View>
      ) : null}
    </View>
  );
};
