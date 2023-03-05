import React, { useState } from "react";
import { Text, TextInput, TouchableOpacity, View } from "react-native";
import { calculateOneRM } from "../../logic/CalucatorsLogic";
import { OneRmStyles } from "../../styles/CalculatorsStyles";
import * as CalcuText from "../../assets/texts/CalculatorTexts";

export const OneRMCalculator: React.FC = () => {
  const [weight, setWeight] = useState<string>("");
  const [reps, setReps] = useState<string>("");
  const [results, setResults] = useState<{ [key: string]: number }>({});

  const calculateResults = () => {
    const w = Number(weight);
    const r = Number(reps);
    const newResults = calculateOneRM(w, r);
    setResults(newResults);
  };

  return (
    <View style={OneRmStyles.container}>
      <Text style={OneRmStyles.label}>{CalcuText.weight}:</Text>
      <TextInput
        style={OneRmStyles.input}
        keyboardType="numeric"
        value={weight}
        onChangeText={setWeight}
      />
      <Text style={OneRmStyles.label}>{CalcuText.reps}:</Text>
      <TextInput
        style={OneRmStyles.input}
        keyboardType="numeric"
        value={reps}
        onChangeText={setReps}
      />
      <TouchableOpacity style={OneRmStyles.button} onPress={calculateResults}>
        <Text style={OneRmStyles.buttonText}>{CalcuText.calculate}</Text>
      </TouchableOpacity>
      <View style={OneRmStyles.rowContainer}>
        <View style={OneRmStyles.rowItem}>
          <Text style={[OneRmStyles.result, OneRmStyles.label]} key={"0"}>
            {CalcuText.method}
          </Text>
          {Object.entries(results).map(([method, result]) => (
            <Text style={OneRmStyles.result} key={method}>
              {`${method}: ${result.toFixed(1)}`}
            </Text>
          ))}
        </View>
        <View style={OneRmStyles.rowItem}>
          <Text style={[OneRmStyles.result, OneRmStyles.label]} key={"0"}>
            {CalcuText.epley}
          </Text>
          {EpleyAllRm(results)}
        </View>
      </View>
    </View>
  );
};

const EpleyAllRm = (results: { [p: string]: number }) => {
  return (
    <View>
      {results["Epley"] && (
        <View>
          <Text style={OneRmStyles.result}>
            {`1RM : ${results["Epley"].toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`2RM : ${(results["Epley"] * 0.94).toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`3RM : ${(results["Epley"] * 0.91).toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`4RM : ${(results["Epley"] * 0.88).toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`5RM : ${(results["Epley"] * 0.86).toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`6RM : ${(results["Epley"] * 0.83).toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`7RM : ${(results["Epley"] * 0.81).toFixed(1)}`}
          </Text>
          <Text style={OneRmStyles.result}>
            {`8RM : ${(results["Epley"] * 0.79).toFixed(1)}`}
          </Text>
        </View>
      )}
    </View>
  );
};
