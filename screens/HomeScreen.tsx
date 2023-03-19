import React, { useEffect } from "react";
import { ScrollView, Text, TouchableOpacity, View } from "react-native";
import { NavigationProp, ParamListBase } from "@react-navigation/native";

import CommonStyles from "../styles/CommonStyles";
import ScreenList, { ScreenItem } from "./ScreenList";
import HomeScreenStyles from "../styles/HomeScreenStyles";
import { createAllTables } from "../database/CommonDB";

interface homeScreenProps {
  navigation: NavigationProp<ParamListBase>;
}

export const HomeScreen: React.FC<homeScreenProps> = ({ navigation }) => {
  useEffect(() => {
    createAllTables(); // call only once when app start
  }, []);
  return (
    <ScrollView>
      <View style={CommonStyles.container}>{ItemsGrid(navigation)}</View>
    </ScrollView>
  );
};

const ItemsGrid = (navigation: NavigationProp<ParamListBase>) => {
  const rows = [];
  for (let i = 0; i < ScreenList.length; i += 2) {
    const item1: ScreenItem = ScreenList[i];
    const item2: ScreenItem = ScreenList[i + 1];

    const row = (
      <View key={i} style={HomeScreenStyles.row}>
        <ListItem navigation={navigation} screenItem={item1} />
        {item2 && <ListItem navigation={navigation} screenItem={item2} />}
      </View>
    );

    rows.push(row);
  }
  return rows;
};

interface ListItemProps {
  navigation: NavigationProp<ParamListBase>;
  screenItem: ScreenItem;
}

const ListItem = ({ navigation, screenItem }: ListItemProps) => {
  const { name, screenName, icon } = screenItem;
  const onPress = () => navigation.navigate(screenName);

  return (
    <TouchableOpacity style={HomeScreenStyles.item} onPress={onPress}>
      {icon}
      <Text style={HomeScreenStyles.itemText}>{name}</Text>
    </TouchableOpacity>
  );
};
