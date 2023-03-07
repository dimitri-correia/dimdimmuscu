import React, { useEffect, useState } from "react";
import { Text, TextInput, TouchableOpacity, View } from "react-native";
import { editInfosEntry, getInfoEntries } from "../database/InfosDB";
import { maxAttemptStyles } from "../styles/CalculatorsStyles";
import DateTimePicker from "@react-native-community/datetimepicker";

export const InfosScreen: React.FC = () => {
  const [name, setName] = useState<string>();
  const [height, setHeight] = useState<string>();
  const [date, setDate] = useState<Date>(new Date());

  useEffect(() => {
    getInfoEntries().then((entries) => {
      setName(entries.get(1)?.fieldValue);
      const dateOfBirth = entries.get(2)?.fieldValue;
      if (dateOfBirth) {
        setDate(new Date(dateOfBirth));
      }
      setHeight(entries.get(3)?.fieldValue);
    });
  }, []);

  const [editDate, setEditDate] = useState<boolean>(false);

  return (
    <View style={maxAttemptStyles.container}>
      <Text style={maxAttemptStyles.title}>{"Name"}:</Text>
      <TextInput
        style={maxAttemptStyles.input}
        value={name}
        onChangeText={(text) => setName(text)}
        onSubmitEditing={() => handleEditing(1, name)}
      />

      <Text style={maxAttemptStyles.title}>{"Date of Birth"}:</Text>

      <TouchableOpacity onPress={() => setEditDate(true)}>
        <Text style={maxAttemptStyles.input}>{date.toDateString()}</Text>
      </TouchableOpacity>
      {editDate && (
        <DateTimePicker
          value={date}
          onChange={(_, dateChange: Date | undefined) => {
            if (dateChange) {
              setEditDate(false);
              setDate(dateChange);
              handleEditing(2, dateChange?.toISOString().split("T")[0]);
            }
          }}
        ></DateTimePicker>
      )}

      <Text style={maxAttemptStyles.title}>{"Height (in cm)"}:</Text>
      <TextInput
        style={maxAttemptStyles.input}
        keyboardType="numeric"
        value={height}
        onChangeText={(text) => setHeight(text)}
        onSubmitEditing={() => handleEditing(3, height)}
      />
    </View>
  );
};

const handleEditing = (id: number, value: string | undefined) => {
  if (!value) {
    return;
  }
  editInfosEntry(id, value);
};
