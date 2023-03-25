import React from "react";
import { Text, View } from "react-native";
import CommonStyles from "../../styles/CommonStyles";
import { pageStyles } from "../../styles/WeightTrackerStyles";
import { useKeepAwake } from "expo-keep-awake";
import Chrono from "../../components/commons/Chrono";

export const SessionTrackerScreenLog: React.FC = (param) => {
  console.log(param);
  useKeepAwake(); // prevent from sleeping
  // table with id, ex, date
  // table with id, idExDate, set, rep, weight
  return (
    <View style={CommonStyles.container}>
      <Chrono />
      <AddNewEx />
      <Row
        setNumber={"#"}
        repDone={"rep"}
        weightLifted={"weight"}
        previous={"previous"}
      />
    </View>
  );
};

interface RowItem {
  setNumber: string;
  repDone: string;
  weightLifted: string;
  previous: string;
}

const Row = ({ setNumber, repDone, weightLifted, previous }: RowItem) => {
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>{setNumber}</Text>
      <Text style={pageStyles.column}>{repDone}</Text>
      <Text style={pageStyles.column}>{weightLifted}</Text>
      <Text style={pageStyles.column}>{previous}</Text>
    </View>
  );
};

const AddNewEx = () => {
  return (
    <View style={pageStyles.row}>
      <Text style={pageStyles.column}>
        {"Add New Exercise for this session"}
      </Text>
    </View>
  );
};
