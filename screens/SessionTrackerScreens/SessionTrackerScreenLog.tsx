import React, { useState } from "react";
import { Button, ScrollView, Text, TextInput, View } from "react-native";
import CommonStyles from "../../styles/CommonStyles";
import { pageStyles } from "../../styles/WeightTrackerStyles";
import { useKeepAwake } from "expo-keep-awake";
import Chrono from "../../components/commons/Chrono";
import { NavigationPropsSessionTrackerPages } from "./SessionTrackerScreen";
import { Picker } from "@react-native-picker/picker";
import { ExercisesEntry } from "../../logic/ExercisesListLogic";
import { getSessionTrackerLiftLastEntry } from "../../logic/SessionTrackerLogic";

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
  //const setList = refreshSessionTrackerSetEntries(ex);

  const setList = [];

  setList?.push({
    id: 1,
    set: 1,
    rep: 2,
    weight: 2,
  });

  console.log(setList);

  return (
    <View>
      <Text>{`${exName ? exName : "ex name"} - previous ${
        lastLift ? lastLift.date.toDateString() : ""
      }`}</Text>

      <RowTitle />
      {setList?.map((set) => {
        console.log(set);
        return (
          <RowItem
            setNumber={set.set.toFixed(0)}
            previous={`${set.rep}*${set.weight}=${set.rep * set.weight}`}
          />
        );
      })}

      <Button
        title={"Add New Set"}
        onPress={() => {
          console.log("new set");
        }}
      />
    </View>
  );
};

interface RowItemProp {
  setNumber: string;
  previous: string;
}

const RowItem = ({ setNumber, previous }: RowItemProp) => {
  const [rep, setRep] = useState<number>(0);
  const [weight, setWeight] = useState<number>(0);
  const total = rep * weight;
  const improvement = total;
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{setNumber}</Text>
      <TextInput style={[pageStyles.column, { backgroundColor: "grey" }]}>
        {rep}
      </TextInput>
      <TextInput style={[pageStyles.column, { backgroundColor: "grey" }]}>
        {weight}
      </TextInput>
      <Text style={pageStyles.column}>{total}</Text>
      <Text style={pageStyles.column}>{previous}</Text>
      <Text style={pageStyles.column}>{improvement}</Text>
    </View>
  );
};
