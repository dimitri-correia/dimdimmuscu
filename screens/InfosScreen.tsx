import React, { useEffect, useState } from "react";
import { Text, TextInput, TouchableOpacity, View } from "react-native";
import { editInfosEntry, getInfoEntries } from "../database/InfosDB";
import { InfoEntry } from "../logic/InfosLogic";
import { maxAttemptStyles } from "../styles/CalculatorsStyles";
import DateTimePicker from "@react-native-community/datetimepicker";

export const InfosScreen: React.FC = () => {
  const [infoEntries, setInfoEntries] = useState<Map<number, InfoEntry>>(
    new Map()
  );

  const [name, setName] = useState<string>("");
  const [date, setDate] = useState<Date>(new Date());
  const [editDate, setEditDate] = useState<boolean>(false);

  useEffect(() => {
    getInfoEntries().then((entries) => setInfoEntries(entries));
  }, []);

  return (
    <View style={maxAttemptStyles.container}>
      <Text style={maxAttemptStyles.title}>{"Name"}:</Text>
      <TextInput
        style={maxAttemptStyles.input}
        keyboardType="numeric"
        value={infoEntries.get(1)?.fieldValue}
        onChangeText={(text) => console.log(text)}
        onSubmitEditing={() => handleEditing(1, "")}
      />

      <Text style={maxAttemptStyles.title}>{"Date of Birth"}:</Text>

      <TouchableOpacity onPress={() => setEditDate(true)}>
        <Text style={maxAttemptStyles.input}>{date.toDateString()}</Text>
      </TouchableOpacity>
      {editDate && (
        <DateTimePicker
          value={date}
          onChange={(_, date: Date | undefined) => {
            setEditDate(false);
            handleEditing(2, date?.toISOString().split("T")[0]);
          }}
        ></DateTimePicker>
      )}

      <Text style={maxAttemptStyles.title}>{"Height (in cm)"}:</Text>
      <TextInput
        style={maxAttemptStyles.input}
        keyboardType="numeric"
        value={infoEntries.get(3)?.fieldValue}
        onChangeText={(text) => console.log(text)}
        onSubmitEditing={() => handleEditing(3, "")}
      />
    </View>
  );
};

function handleEditing(id: number, value: string | undefined) {
  if (!value) {
    return;
  }
  editInfosEntry(id, value);
}
