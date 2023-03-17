import React, { useEffect, useState } from "react";
import {
  Button,
  Modal,
  ScrollView,
  Text,
  TextInput,
  TouchableOpacity,
  View,
} from "react-native";
import ExercisesListStyles from "../styles/ExercisesListStyles";
import {
  addExercise,
  addMuscleGroup,
  ExercisesEntry,
  refreshExercisesAndMuscleEntries,
} from "../logic/ExercisesListLogic";
import * as TextEL from "../assets/texts/ExercisesListTexts";
import CommonStyles, { editWeightStyles } from "../styles/CommonStyles";
import { Picker } from "@react-native-picker/picker";
import * as TextCommon from "../assets/texts/Common";

export const ExercisesList: React.FC = () => {
  const [exerciseEntries, setExerciseEntries] = useState<ExercisesEntry[]>([]);
  const [muscleGroupEntries, setMuscleGroupEntries] = useState<
    Map<number, string>
  >(new Map());

  const [modalEx, setModalEx] = useState<boolean>(false);
  const [modalExName, setModalExName] = useState<string>("");
  const [modalExPrimary, setModalExPrimary] = useState<number>(1);
  const [modalExSecondary, setModalExSecondary] = useState<number>(0);

  const [modalGM, setModalGM] = useState<boolean>(false);
  const [modalGMName, setModalGMName] = useState<string>("");

  useEffect(() => {
    refreshExercisesAndMuscleEntries(setExerciseEntries, setMuscleGroupEntries);
  }, [modalEx, modalGM]);

  return (
    <View style={CommonStyles.container}>
      <ModalAddMuscleGroup
        modalGM={modalGM}
        setModalGM={setModalGM}
        setModalGMName={setModalGMName}
        handleAddMuscleGroup={() =>
          addMuscleGroup(setModalGM, modalGMName, muscleGroupEntries)
        }
      />
      <ModalAddExercise
        modalEx={modalEx}
        setModalEx={setModalEx}
        setModalExName={setModalExName}
        modalExPrimary={modalExPrimary}
        setModalExPrimary={setModalExPrimary}
        modalExSecondary={modalExSecondary}
        setModalExSecondary={setModalExSecondary}
        muscleGroupEntries={muscleGroupEntries}
        handleAddExercise={() =>
          addExercise(
            setModalEx,
            modalExName,
            modalExPrimary,
            modalExSecondary,
            exerciseEntries
          )
        }
      />
      <Button title={TextEL.addExercise} onPress={() => setModalEx(true)} />
      <ScrollView>
        <View style={ExercisesListStyles.row}>
          <Text style={ExercisesListStyles.header}>{TextEL.exerciseName}</Text>
          <Text style={ExercisesListStyles.header}>
            {TextEL.primaryMuscleGroup}
          </Text>
          <Text style={ExercisesListStyles.header}>
            {TextEL.secondaryMuscleGroup}
          </Text>
        </View>
        {exerciseEntries
          .sort((entry) => entry.primaryMuscleGroupId)
          .map((entry: ExercisesEntry) => (
            <View style={ExercisesListStyles.row} key={entry.id}>
              <Text style={[ExercisesListStyles.item, { flex: 2 }]}>
                {entry.name}
              </Text>
              <Text style={ExercisesListStyles.item}>
                {muscleGroupEntries.get(entry.primaryMuscleGroupId)}
              </Text>
              <Text style={ExercisesListStyles.item}>
                {muscleGroupEntries.get(entry.secondaryMuscleGroupId)}
              </Text>
            </View>
          ))}
      </ScrollView>

      <Button title={TextEL.addMuscleGroup} onPress={() => setModalGM(true)} />
      <ScrollView>
        <View style={ExercisesListStyles.row}>
          <Text style={ExercisesListStyles.header}>Name</Text>
        </View>
        {Array.from(muscleGroupEntries.values())
          .filter((a, id) => id != 0) // filter the empty value (id=0)
          .reduce((rows: string[][], entry: string, index: number) => {
            if (index % 3 === 0) {
              rows.push([entry]);
            } else {
              rows[rows.length - 1].push(entry);
            }
            return rows;
          }, [])
          .map((row: string[], rowIndex: number) => (
            <View style={ExercisesListStyles.row} key={rowIndex}>
              {row.map((entry: string, entryIndex: number) => (
                <Text style={ExercisesListStyles.item} key={entryIndex}>
                  {entry}
                </Text>
              ))}
            </View>
          ))}
      </ScrollView>
    </View>
  );
};

interface ModalAddMuscleGroupProps {
  modalGM: boolean;
  setModalGM: (value: ((prevState: boolean) => boolean) | boolean) => void;
  setModalGMName: (value: ((prevState: string) => string) | string) => void;
  handleAddMuscleGroup: () => void;
}

const ModalAddMuscleGroup = ({
  modalGM,
  setModalGM,
  setModalGMName,
  handleAddMuscleGroup,
}: ModalAddMuscleGroupProps) => {
  return (
    <Modal animationType="slide" transparent={true} visible={modalGM}>
      <View style={editWeightStyles.modal}>
        <View style={editWeightStyles.background}>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.title}>{TextEL.addMuscleGroup}</Text>
          </View>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.label}>{TextEL.muscleGroupName}</Text>
            <TextInput
              style={editWeightStyles.input}
              onChangeText={(text) => setModalGMName(text)}
            />
          </View>
          <View style={editWeightStyles.line}>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonOK]}
              onPress={handleAddMuscleGroup}
            >
              <Text style={editWeightStyles.buttonText}>{TextCommon.save}</Text>
            </TouchableOpacity>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonKO]}
              onPress={() => setModalGM(false)}
            >
              <Text style={editWeightStyles.buttonText}>
                {TextCommon.cancel}
              </Text>
            </TouchableOpacity>
          </View>
        </View>
      </View>
    </Modal>
  );
};

interface ModalAddExerciseProps {
  handleAddExercise: () => void;
  modalEx: boolean;
  modalExPrimary: number;
  modalExSecondary: number;
  muscleGroupEntries: Map<number, string>;
  setModalEx: (value: ((prevState: boolean) => boolean) | boolean) => void;
  setModalExName: (value: ((prevState: string) => string) | string) => void;
  setModalExPrimary: (value: ((prevState: number) => number) | number) => void;
  setModalExSecondary: (
    value: ((prevState: number) => number) | number
  ) => void;
}

const ModalAddExercise = ({
  modalEx,
  setModalEx,
  setModalExName,
  modalExPrimary,
  setModalExPrimary,
  modalExSecondary,
  setModalExSecondary,
  muscleGroupEntries,
  handleAddExercise,
}: ModalAddExerciseProps) => {
  return (
    <Modal animationType="slide" transparent={true} visible={modalEx}>
      <View style={editWeightStyles.modal}>
        <View style={editWeightStyles.background}>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.title}>{TextEL.addExercise}</Text>
          </View>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.label}>{TextEL.exerciseName}</Text>
            <TextInput
              style={editWeightStyles.input}
              onChangeText={(text) => setModalExName(text)}
            />
          </View>

          <Text>{TextEL.primaryMuscleGroup}</Text>
          <Picker
            selectedValue={modalExPrimary}
            onValueChange={(value: number) => setModalExPrimary(value)}
          >
            {Array.from(muscleGroupEntries.entries())
              .filter(([id]) => id != 0) // id = 0 is empty muscle group
              .map(([id, name]) => (
                <Picker.Item key={id} label={name} value={id} />
              ))}
          </Picker>

          <Text>{TextEL.secondaryMuscleGroup}</Text>
          <Picker
            selectedValue={modalExSecondary}
            onValueChange={(value: number) => setModalExSecondary(value)}
          >
            {Array.from(muscleGroupEntries.entries()).map(([id, name]) => (
              <Picker.Item key={id} label={name} value={id} />
            ))}
          </Picker>

          <View style={editWeightStyles.line}>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonOK]}
              onPress={handleAddExercise}
            >
              <Text style={editWeightStyles.buttonText}>{TextCommon.save}</Text>
            </TouchableOpacity>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonKO]}
              onPress={() => setModalEx(false)}
            >
              <Text style={editWeightStyles.buttonText}>
                {TextCommon.cancel}
              </Text>
            </TouchableOpacity>
          </View>
        </View>
      </View>
    </Modal>
  );
};
