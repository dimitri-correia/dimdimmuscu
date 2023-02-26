import React, { useEffect, useState } from "react";
import { getPREntries } from "../database/PersonalRecordDB";
import { PersonalRecordEntry } from "../logic/PersonalRecordLogic";
import { Text, View } from "react-native";

export const PersonalRecordScreen: React.FC = () => {
  const [personalRecordEntries, setPersonalRecordEntries] = useState<
    PersonalRecordEntry[]
  >([]);

  useEffect(() => {
    refreshPREntries(setPersonalRecordEntries);
  }, []);

  const exIds = Array.from(
    new Set(personalRecordEntries.map((entry) => entry.exId))
  );

  return (
    <View>
      {exIds.map((exId) => (
        <ExerciseTable key={exId} exId={exId} entries={personalRecordEntries} />
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
    <View>
      <Text>{`Exercise ${exId}`}</Text>
      <View>
        <View style={{ flexDirection: "row" }}>
          <Text style={{ flex: 1 }}>Date</Text>
          <Text style={{ flex: 1 }}>Weight</Text>
          <Text style={{ flex: 1 }}>Improvement</Text>
        </View>
        {entries
          .filter((entry) => entry.exId === exId)
          .map((entry) => (
            <View key={entry.id} style={{ flexDirection: "row" }}>
              <Text style={{ flex: 1 }}>{entry.date.toDateString()}</Text>
              <Text style={{ flex: 1 }}>{entry.weightLifted}</Text>
              <Text style={{ flex: 1 }}>{0}</Text>
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
