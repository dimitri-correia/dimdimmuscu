import React, { useEffect, useState } from "react";
import { Button, ScrollView, Text, View } from "react-native";
import CommonStyles from "../../styles/CommonStyles";
import { pageStyles } from "../../styles/WeightTrackerStyles";
import { useKeepAwake } from "expo-keep-awake";
import Chrono from "../../components/commons/Chrono";
import { NavigationPropsSessionTrackerPages } from "./SessionTrackerScreen";
import { Picker } from "@react-native-picker/picker";
import { ExercisesEntry } from "../../logic/ExercisesListLogic";
import {
  getSessionTrackerLiftLastEntry,
  refreshSessionTrackerLiftEntries,
  refreshSessionTrackerSetEntries,
  SessionTrackerLiftEntry,
} from "../../logic/SessionTrackerLogic";

export const SessionTrackerScreenLog: React.FC<
  NavigationPropsSessionTrackerPages
> = (param) => {
  const exList: ExercisesEntry[] = param.route.params.ex;
  useKeepAwake(); // prevent from sleeping

  const [exToShow, setExToShow] = useState<number[]>([1, 2]);

  return (
    <ScrollView style={CommonStyles.container}>
      <Chrono />
      <AddNewEx
        exerciseList={exList}
        exToShow={exToShow}
        setExToShow={setExToShow}
      />
      {exToShow.reverse().map((ex) => (
        <Ex exName={exList.find((e) => e.id === ex)?.name} ex={ex} />
      ))}
    </ScrollView>
  );
};

interface RowItem {
  setNumber: string;
  repDone: string;
  weightLifted: string;
  total: string;
  previous: string;
  improvement: string;
}

const Row = ({
  setNumber,
  repDone,
  weightLifted,
  total,
  previous,
  improvement,
}: RowItem) => {
  return (
    <View>
      <View style={pageStyles.row}>
        <Text style={pageStyles.column}>{setNumber}</Text>
        <Text style={pageStyles.column}>{repDone}</Text>
        <Text style={pageStyles.column}>{weightLifted}</Text>
        <Text style={pageStyles.column}>{total}</Text>
        <Text style={pageStyles.column}>{previous}</Text>
        <Text style={pageStyles.column}>{improvement}</Text>
      </View>
    </View>
  );
};

interface AddNewExProps {
  exerciseList: ExercisesEntry[];
  exToShow: number[];
  setExToShow: (value: ((prevState: number[]) => number[]) | number[]) => void;
}

const AddNewEx = ({ exerciseList, exToShow, setExToShow }: AddNewExProps) => {
  const [ex, setEx] = useState<number>(0);
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>
        {"Add New Exercise for this session"}
      </Text>
      <Text style={{ flex: 1 }}>
        {exerciseList.find((ee) => ee.id == ex)?.name}
      </Text>
      <Picker
        style={{ flex: 1 }}
        selectedValue={ex}
        onValueChange={(value: number) => setEx(value)}
      >
        {exerciseList.map((ex) => (
          <Picker.Item key={ex.id} label={ex.name} value={ex.id} />
        ))}
      </Picker>
      <Button
        title={"Add"}
        onPress={() => {
          exToShow.push(ex);
          setExToShow(exToShow);
        }}
      />
    </View>
  );
};

interface ExProps {
  exName: string | undefined;
  ex: number;
}

const Ex = ({ exName, ex }: ExProps) => {
  const lastLift = getSessionTrackerLiftLastEntry(ex);
  const setList = refreshSessionTrackerSetEntries(ex);
  return (
    <View>
      <Text>{`${exName ? exName : "ex name"} - previous ${
        lastLift ? lastLift.date.toDateString() : ""
      }`}</Text>
      <Row
        setNumber={"#"}
        repDone={"rep"}
        weightLifted={"weight"}
        total={"total"}
        previous={"previous"}
        improvement={"improvement"}
      />
      {setList?.map((set) => (
        <Row
          setNumber={set.set.toFixed(0)}
          repDone={"0"}
          weightLifted={"0"}
          total={"0"}
          previous={`${set.rep}*${set.weight}=${set.rep * set.weight}`}
          improvement={"0"}
        />
      ))}
    </View>
  );
};
