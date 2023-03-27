import React, { useState } from "react";
import { Button, ScrollView, Text, TextInput, View } from "react-native";
import CommonStyles from "../../styles/CommonStyles";
import { pageStyles } from "../../styles/WeightTrackerStyles";
import { useKeepAwake } from "expo-keep-awake";
import Chrono from "../../components/commons/Chrono";
import { NavigationPropsSessionTrackerPages } from "./SessionTrackerScreen";
import { Picker } from "@react-native-picker/picker";
import { ExercisesEntry } from "../../logic/ExercisesListLogic";
import {
  getSessionTrackerLiftLastEntry,
  SessionTrackerSetEntry,
} from "../../logic/SessionTrackerLogic";
import { getImprovementString } from "../../logic/PersonalRecordLogic";
import { addSessionTrackerSetEntry } from "../../database/SessionTrackerDB";

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

const RowTitle = () => {
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{"#"}</Text>
      <Text style={pageStyles.column}>{"rep"}</Text>
      <Text style={pageStyles.column}>{"weight"}</Text>
      <Text style={pageStyles.column}>{"tot"}</Text>
      <Text style={pageStyles.column}>{"prev"}</Text>
      <Text style={pageStyles.column}>{"improv"}</Text>
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
  //const setList = refreshSessionTrackerSetEntries(lastLift?.id);

  const setList = [] as SessionTrackerSetEntry[];

  setList?.push({
    id: 1,
    set: 1,
    rep: 2,
    weight: 2,
  });

  console.log(setList);

  const maxSet = setList.sort((s) => s.set).at(0)?.set;

  const newSet = maxSet ? maxSet + 1 : 1;

  return (
    <View>
      <Text>{`${exName ? exName : "ex name"} - previous ${
        lastLift ? lastLift.date.toDateString() : ""
      }`}</Text>

      <RowTitle />
      {setList?.map((set) => {
        return (
          <RowItem
            setNumber={set.set.toFixed(0)}
            previousRep={set.rep}
            previousWeight={set.weight}
          />
        );
      })}

      <Button
        title={"Add New Set"}
        onPress={() => {
          addSessionTrackerSetEntry(
            lastLift?.id ? lastLift.id : -1,
            newSet,
            0,
            0
          )
            .then((r) => {
              setList.push({
                id: r,
                set: newSet,
                rep: 0,
                weight: 0,
              });
            })
            .catch(() => console.log("Error adding new set"));
        }}
      />
    </View>
  );
};

interface RowItemProp {
  setNumber: string;
  previousRep: number;
  previousWeight: number;
}

const RowItem = ({ setNumber, previousRep, previousWeight }: RowItemProp) => {
  const [rep, setRep] = useState<number>(0);
  const [weight, setWeight] = useState<number>(0);
  const total = rep * weight;
  const totalPrevious = previousWeight * previousRep;
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{setNumber}</Text>
      <TextInput
        style={[pageStyles.column, { backgroundColor: "grey" }]}
        keyboardType="numeric"
        onChangeText={(text) => setRep(Number(text))}
      >
        {rep}
      </TextInput>
      <TextInput
        style={[pageStyles.column, { backgroundColor: "grey" }]}
        keyboardType="numeric"
        onChangeText={(text) => setWeight(Number(text))}
      >
        {weight}
      </TextInput>
      <Text style={pageStyles.column}>{total}</Text>
      <Text
        style={pageStyles.column}
      >{`${previousRep}*${previousWeight}=${totalPrevious}`}</Text>
      <Text style={pageStyles.column}>
        {getImprovementString(total, totalPrevious)}
      </Text>
    </View>
  );
};
