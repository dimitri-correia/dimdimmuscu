import React, { useState } from "react";
import { Button, Text, View } from "react-native";
import CommonStyles from "../../styles/CommonStyles";
import { pageStyles } from "../../styles/WeightTrackerStyles";
import { useKeepAwake } from "expo-keep-awake";
import Chrono from "../../components/commons/Chrono";
import { NavigationPropsSessionTrackerPages } from "./SessionTrackerScreen";
import { Picker } from "@react-native-picker/picker";
import { ExercisesEntry } from "../../logic/ExercisesListLogic";

export const SessionTrackerScreenLog: React.FC<
  NavigationPropsSessionTrackerPages
> = (param) => {
  console.log(param.route.params.ex);
  const exlist: ExercisesEntry[] = param.route.params.ex;
  useKeepAwake(); // prevent from sleeping
  // table with id, ex, date
  // table with id, idExDate, set, rep, weight
  return (
    <View style={CommonStyles.container}>
      <Chrono />
      <AddNewEx exerciseList={exlist} />
      <Ex />
      <Ex />
    </View>
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
      </View>
      <View style={pageStyles.row}>
        <Text style={pageStyles.column}>{total}</Text>
        <Text style={pageStyles.column}>{previous}</Text>
        <Text style={pageStyles.column}>{improvement}</Text>
      </View>
    </View>
  );
};

interface AddNewExProps {
  exerciseList: ExercisesEntry[];
}

const AddNewEx = ({ exerciseList }: AddNewExProps) => {
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
      <Button title={"Add"} onPress={() => {}} />
    </View>
  );
};

const Ex = () => {
  return (
    <View>
      <Text>{"Ex name"}</Text>
      <Row
        setNumber={"#"}
        repDone={"rep"}
        weightLifted={"weight"}
        total={"total"}
        previous={"previous"}
        improvement={"improvement"}
      />
    </View>
  );
};
