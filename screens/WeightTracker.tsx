import React, { useEffect, useState } from "react";
import {
  Button,
  Modal,
  Text,
  TextInput,
  TouchableOpacity,
  View,
} from "react-native";
import { averageXdays, WeightEntry } from "../logic/WeightTrackerLogic";
import {
  addWeightEntry,
  deleteWeightEntry,
  editWeightEntry,
  getWeightEntries,
} from "../database/WeightTrackerDB";
import CommonStyles, { editWeightStyles } from "../styles/CommonStyles";
import { addWeightStyles, pageStyles } from "../styles/WeightTrackerStyles";
import * as TextWT from "../assets/texts/WeightTracker";
import { confirmationChanges } from "../components/commons/ValidateChanges";

export const WeightTracker: React.FC = () => {
  const [weightEntries, setWeightEntries] = useState<WeightEntry[]>([]);

  const [newWeight, setNewWeight] = useState<string>("");
  const handleAddWeight = addWeight(weightEntries, newWeight);

  const [editId, setEditId] = useState<number>(-1); // if -1 modal invisible
  const handleEditWeight = editEntry(newWeight, editId, setEditId);
  const handleDeleteWeight = deleteEntry(editId, setEditId);

  useEffect(() => {
    refreshWeightEntries(setWeightEntries); // fetch database once
  }, [editId, handleAddWeight]);

  return (
    <View style={CommonStyles.container}>
      <ModalEditWeight
        editId={editId}
        setNewWeight={setNewWeight}
        handleEditWeight={handleEditWeight}
        handleDeleteWeight={handleDeleteWeight}
      />
      <View style={addWeightStyles.addWeightContainer}>
        <Text style={addWeightStyles.addWeightLabel}>{TextWT.enterWeight}</Text>
        <TextInput
          style={addWeightStyles.addWeightInput}
          keyboardType="numeric"
          onChangeText={(text) => setNewWeight(text)}
        />
        <Button title={TextWT.add} onPress={handleAddWeight} />
      </View>
      <Row
        date={TextWT.date}
        todayWeight={TextWT.weightInKg}
        average7days={TextWT.average7}
        average31days={TextWT.average31}
      />
      {weightEntries.map((entry) => (
        <TouchableOpacity
          key={entry.date.toISOString()}
          onPress={() => {
            setEditId(entry.id);
          }}
        >
          <Row
            date={entry.date.toDateString()}
            todayWeight={entry.weight.toFixed(1)}
            average7days={averageXdays(weightEntries, entry.date, 7).toFixed(1)}
            average31days={averageXdays(weightEntries, entry.date, 31).toFixed(
              1
            )}
          />
        </TouchableOpacity>
      ))}
    </View>
  );
};

interface RowItem {
  date: string;
  todayWeight: string;
  average7days: string;
  average31days: string;
}

const Row = ({ date, todayWeight, average7days, average31days }: RowItem) => {
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{date}</Text>
      <Text style={pageStyles.column}>{todayWeight}</Text>
      <Text style={pageStyles.column}>{average7days}</Text>
      <Text style={pageStyles.column}>{average31days}</Text>
    </View>
  );
};

interface ModalEditProps {
  editId: number;
  setNewWeight: any;
  handleEditWeight: any;
  handleDeleteWeight: any;
}

const ModalEditWeight = ({
  editId,
  setNewWeight,
  handleEditWeight,
  handleDeleteWeight,
}: ModalEditProps) => {
  return (
    <Modal animationType="slide" transparent={true} visible={editId != -1}>
      <View style={editWeightStyles.modal}>
        <View style={editWeightStyles.background}>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.title}>Edit Weight Entry</Text>
          </View>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.label}>{TextWT.enterWeight}</Text>
            <TextInput
              style={editWeightStyles.input}
              keyboardType="numeric"
              onChangeText={(text) => setNewWeight(text)}
            />
          </View>
          <View style={editWeightStyles.line}>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonOK]}
              onPress={handleEditWeight}
            >
              <Text style={editWeightStyles.buttonText}>Save</Text>
            </TouchableOpacity>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonKO]}
              onPress={handleDeleteWeight}
            >
              <Text style={editWeightStyles.buttonText}>Delete</Text>
            </TouchableOpacity>
          </View>
        </View>
      </View>
    </Modal>
  );
};

function refreshWeightEntries(
  setWeightEntries: (
    value: ((prevState: WeightEntry[]) => WeightEntry[]) | WeightEntry[]
  ) => void
) {
  getWeightEntries().then((we: WeightEntry[]) => {
    setWeightEntries(we);
  });
}

function editEntry(
  modifiedWeight: string,
  editId: number,
  setEditId: (value: ((prevState: number) => number) | number) => void
) {
  return () => {
    const parsedWeight = parseFloat(modifiedWeight);
    if (isNaN(parsedWeight)) {
      alert(TextWT.invalidWeight);
      return;
    }
    confirmationChanges(() => {
      editWeightEntry(editId, parsedWeight);
      setEditId(-1);
    });
  };
}

function deleteEntry(
  editId: number,
  setEditId: (value: ((prevState: number) => number) | number) => void
) {
  return () => {
    confirmationChanges(() => {
      deleteWeightEntry(editId);
      setEditId(-1);
    });
  };
}

function addWeight(weightEntries: WeightEntry[], newWeight: string) {
  return () => {
    const today: string = new Date().toISOString().split("T")[0];
    const hasEntryForToday: boolean = weightEntries.some(
      (entry: WeightEntry) => entry.date.toISOString().split("T")[0] === today
    );

    if (hasEntryForToday) {
      alert(TextWT.alreadyExistingWeight);
    }

    const parsedWeight = parseFloat(newWeight);
    if (isNaN(parsedWeight)) {
      alert(TextWT.invalidWeight);
      return;
    }
    addWeightEntry(today, parsedWeight);
  };
}
