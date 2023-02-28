import React, { useEffect, useState } from "react";
import { getPREntries } from "../database/PersonalRecordDB";
import { PersonalRecordEntry } from "../logic/PersonalRecordLogic";
import { Text, View } from "react-native";

export const PersonalRecordScreen: React.FC = () => {
  const [personalRecordEntries, setPersonalRecordEntries] = useState<
    PersonalRecordEntry[]
  >([]);

  const [modalShow, setModalShow] = useState<boolean>(false);

  useEffect(() => {
    refreshPREntries(setPersonalRecordEntries);
  }, []);

  const entriesByExercise = personalRecordEntries.reduce((acc, entry) => {
    const exId = entry.exId;
    if (!acc[exId]) {
      acc[exId] = [];
    }
    acc[exId].push(entry);
    return acc;
  }, {} as Record<number, PersonalRecordEntry[]>);

  return (
    <View style={CommonStyles.container}>
      <ModalAddPR modalShow={modalShow} setModalShow={setModalShow} />

      <Button title={TextEL.addExercise} onPress={() => setModalShow(true)} />
      {Object.entries(entriesByExercise).map(([exId, entries]) => (
        <ExerciseTable key={exId} exId={Number(exId)} entries={entries} />
      ))}
    </View>
  );
};

interface ExerciseTableProps {
  exId: number;
  entries: PersonalRecordEntry[];
}

const ExerciseTable = ({ exId, entries }: ExerciseTableProps) => {
  return (
    <View key={exId} style={PersonalRecordStyles.exTable}>
      <Text style={PersonalRecordStyles.tableName}>{`Exercise ${exId}`}</Text>
      <View>
        <View style={PersonalRecordStyles.tableLines}>
          <Text style={PersonalRecordStyles.tableItem}>{TextPR.date}</Text>
          <Text style={PersonalRecordStyles.tableItem}>{TextPR.weight}</Text>
          <Text style={PersonalRecordStyles.tableItem}>
            {TextPR.improvement}
          </Text>
        </View>
        {entries.map((entry, p) => (
          <View key={entry.id} style={PersonalRecordStyles.tableLines}>
            <Text style={PersonalRecordStyles.tableItem}>
              {entry.date.toDateString()}
            </Text>
            <Text style={PersonalRecordStyles.tableItem}>
              {entry.weightLifted}
            </Text>
            <Text style={PersonalRecordStyles.tableItem}>
              {getImprovementString(
                entry.weightLifted,
                entries.at(p - 1).weightLifted
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
      | ((prevState: PersonalRecordEntry[]) => PersonalRecordEntry[])
      | PersonalRecordEntry[]
  ) => void
) {
  let prr: PersonalRecordEntry[] = [];
  const number = prr.push(
    {
      id: 1,
      weightLifted: 100,
      date: new Date(),
      exId: 1,
    },
    {
      id: 2,
      weightLifted: 102,
      date: new Date(),
      exId: 1,
    },
    {
      id: 3,
      weightLifted: 10,
      date: new Date(),
      exId: 2,
    },
    {
      id: 4,
      weightLifted: 105,
      date: new Date(),
      exId: 1,
    }
  );
  console.log(number);
  console.log(prr);
  setPersonalRecordEntries(prr);
  return;
  getPREntries().then((pr) => {
    setPersonalRecordEntries(pr);
  });
}
