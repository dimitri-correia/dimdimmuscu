import React, { useEffect, useState } from "react";
import { addPREntry, getPREntries } from "../database/PersonalRecordDB";
import {
  getImprovementString,
  PersonalRecordEntry,
} from "../logic/PersonalRecordLogic";
import {
  Button,
  Modal,
  Text,
  TextInput,
  TouchableOpacity,
  View,
} from "react-native";
import DateTimePicker from "@react-native-community/datetimepicker";
import CommonStyles, { editWeightStyles } from "../styles/CommonStyles";
import PersonalRecordStyles from "../styles/PersonalRecordStyles";
import * as TextPR from "../assets/texts/PersonalRecordTexts";
import { Picker } from "@react-native-picker/picker";
import * as TextCommon from "../assets/texts/Common";
import { getExerciseEntries } from "../database/ExercisesListDB";
import { ExercisesEntry } from "../logic/ExercisesListLogic";
import { confirmationChanges } from "../components/commons/ValidateChanges";
import { addWeightStyles } from "../styles/WeightTrackerStyles";

export const PersonalRecordScreen: React.FC = () => {
  const [personalRecordEntries, setPersonalRecordEntries] = useState<{
    [key: number]: PersonalRecordEntry[];
  }>({});

  const [modalShow, setModalShow] = useState<boolean>(false);

  const [exerciseEntries, setExerciseEntries] = useState<ExercisesEntry[]>([]);

  const [modalExToSet, setModalExToSet] = useState<number>(1);
  const [modalWeightToSet, setModalWeightToSet] = useState<number>(0);
  const [modalDate, setModalDate] = useState<Date>(new Date());

  useEffect(() => {
    refreshPREntries(setPersonalRecordEntries);
  }, [modalShow]);

  useEffect(() => {
    getExerciseEntries()
      .then((ex: ExercisesEntry[]) => {
        setExerciseEntries(ex);
      })
      .catch(() => {
        console.debug("error fetching ex entries");
      });
  }, []);

  return (
    <View style={CommonStyles.container}>
      <ModalAddPR
        modalShow={modalShow}
        setModalShow={setModalShow}
        exerciseList={exerciseEntries}
        handleAddPR={() =>
          addPR(
            modalDate.toISOString().split("T")[0],
            modalExToSet,
            modalWeightToSet,
            setModalShow
          )
        }
        modalExToSet={modalExToSet}
        setModalExToSet={setModalExToSet}
        setModalWeightToSet={setModalWeightToSet}
        modalDate={modalDate}
        setModalDate={setModalDate}
      />

      <Button title={"Add a new PR"} onPress={() => setModalShow(true)} />
      {Object.entries(personalRecordEntries).map(([exerciseId, entries]) => (
        <ExerciseTable
          exName={
            exerciseEntries.find((ex) => ex.id == Number(exerciseId))?.name
          }
          entries={entries}
        />
      ))}
    </View>
  );
};

interface ModalAddPRProps {
  modalShow: boolean;
  setModalShow: (value: ((prevState: boolean) => boolean) | boolean) => void;
  exerciseList: ExercisesEntry[] | undefined;
  handleAddPR: () => void;
  modalExToSet: number;
  setModalExToSet: (value: ((prevState: number) => number) | number) => void;
  setModalWeightToSet: (
    value: ((prevState: number) => number) | number
  ) => void;
  modalDate: Date;
  setModalDate: (value: ((prevState: Date) => Date) | Date) => void;
}

const ModalAddPR = ({
  modalShow,
  setModalShow,
  exerciseList,
  handleAddPR,
  modalExToSet,
  setModalExToSet,
  setModalWeightToSet,
  modalDate,
  setModalDate,
}: ModalAddPRProps) => {
  if (exerciseList == undefined) {
    return <View />;
  }
  const [editDate, setEditDate] = useState<boolean>(false);
  return (
    <Modal animationType="slide" transparent={true} visible={modalShow}>
      <View style={editWeightStyles.modal}>
        <View style={editWeightStyles.background}>
          <View style={editWeightStyles.line}>
            <Text style={editWeightStyles.title}>Add a new PR</Text>
          </View>
          <View style={editWeightStyles.line}>
            <Button
              title={modalDate.toDateString()}
              onPress={() => setEditDate(true)}
            />
            {editDate && (
              <DateTimePicker
                value={modalDate}
                onChange={(_, date: Date | undefined) => {
                  setEditDate(false);
                  if (date) setModalDate(date);
                }}
              ></DateTimePicker>
            )}
          </View>

          <View style={editWeightStyles.line}>
            <Text style={{ flex: 1 }}>
              {exerciseList.find((ex) => ex.id == modalExToSet)?.name}
            </Text>
            <Picker
              style={{ flex: 1 }}
              selectedValue={modalExToSet}
              onValueChange={(value: number) => setModalExToSet(value)}
            >
              {exerciseList.map((ex) => (
                <Picker.Item key={ex.id} label={ex.name} value={ex.id} />
              ))}
            </Picker>
          </View>

          <View style={editWeightStyles.line}>
            <TextInput
              style={addWeightStyles.addWeightInput}
              keyboardType="numeric"
              placeholder={"weigth"}
              onChangeText={(text) => setModalWeightToSet(Number(text))}
            />
          </View>

          <View style={editWeightStyles.line}>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonOK]}
              onPress={handleAddPR}
            >
              <Text style={editWeightStyles.buttonText}>{TextCommon.save}</Text>
            </TouchableOpacity>
            <TouchableOpacity
              style={[editWeightStyles.button, editWeightStyles.buttonKO]}
              onPress={() => setModalShow(false)}
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

interface ExerciseTableProps {
  exName: string | undefined;
  entries: PersonalRecordEntry[];
}

const ExerciseTable = ({ exName, entries }: ExerciseTableProps) => {
  if (!exName) {
    return <View />;
  }
  return (
    <View key={exName} style={PersonalRecordStyles.exTable}>
      <Text
        style={PersonalRecordStyles.tableName}
      >{`Exercise: ${exName}`}</Text>
      <View>
        <View style={PersonalRecordStyles.tableLines}>
          <Text style={PersonalRecordStyles.tableItem}>{TextPR.date}</Text>
          <Text style={PersonalRecordStyles.tableItem}>{TextPR.weight}</Text>
          <Text style={PersonalRecordStyles.tableItem}>
            {TextPR.improvement}
          </Text>
        </View>
        {entries.map((entry, index) => (
          <View key={entry.id} style={PersonalRecordStyles.tableLines}>
            <Text style={PersonalRecordStyles.tableItem}>
              {entry.date.toDateString()}
            </Text>
            <Text style={PersonalRecordStyles.tableItem}>
              {entry.weightLifted} kg
            </Text>
            <Text style={PersonalRecordStyles.tableItem}>
              {getImprovementString(
                entry.weightLifted,
                entries[index + 1]?.weightLifted
              )}
            </Text>
          </View>
        ))}
      </View>
    </View>
  );
};

function refreshPREntries(
  setPersonalRecordEntries: (
    value:
      | ((prevState: { [p: string]: PersonalRecordEntry[] }) => {
          [p: string]: PersonalRecordEntry[];
        })
      | { [p: string]: PersonalRecordEntry[] }
  ) => void
) {
  getPREntries()
    .then((pr) => {
      const exercises: { [key: number]: PersonalRecordEntry[] } = {};
      // group the entries by exercise name
      pr.forEach((entry: PersonalRecordEntry) => {
        if (!exercises[entry.exId]) {
          exercises[entry.exId] = [];
        }
        exercises[entry.exId].push(entry);
      });
      setPersonalRecordEntries(exercises);
    })
    .catch(() => {
      console.debug("error fetching pr entries");
    });
}

function addPR(
  date: string,
  exoID: number,
  weight: number,
  setModal: (value: ((prevState: boolean) => boolean) | boolean) => void
) {
  confirmationChanges(() => {
    addPREntry(date, exoID, weight);
    setModal(false);
  });
}
